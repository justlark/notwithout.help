use std::fmt;

use chrono::NaiveDateTime;
use serde::Deserialize;
use worker::{d1::D1Database, query};

use crate::{
    keys::{EphemeralServerKey, PublicSigningKey, WrappedPrivatePrimaryKey},
    models::{
        ClientKeyId, ClientKeys, EncryptedKeyComment, EncryptedSubmissionBody, FormId,
        FormTemplate, ServerKeyId, Submission, SubmissionId,
    },
};

// SQLite natively understands datetime strings with this format; it uses the format when
// automatically generating timestamps with `DEFAULT CURRENT_TIMESTAMP`.
const SQLITE_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct Store {
    db: D1Database,
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store").finish_non_exhaustive()
    }
}

impl Store {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }
}

impl Store {
    #[worker::send]
    pub async fn get_form_template(&self, form_id: FormId) -> anyhow::Result<Option<FormTemplate>> {
        let stmt = query!(
            &self.db,
            "
            SELECT template
            FROM forms
            WHERE form_id = ?1;
            ",
            form_id,
        )?;

        Ok(stmt
            .first::<String>(Some("template"))
            .await?
            .map(|raw| serde_json::from_str(&raw))
            .transpose()?)
    }

    #[worker::send]
    pub async fn put_form_template(
        &self,
        form_id: FormId,
        template: FormTemplate,
    ) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO forms (form_id, template)
            VALUES (?1, ?2);
            ",
            form_id,
            serde_json::to_string(&template)?,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_form(&self, form_id: FormId) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            DELETE FROM forms
            WHERE forms.form_id = ?1;
            ",
            form_id,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn list_submissions(&self, form_id: FormId) -> anyhow::Result<Vec<Submission>> {
        let stmt = query!(
            &self.db,
            "
            SELECT submissions.encrypted_body, submissions.created_at
            FROM submissions
            JOIN forms ON submissions.form = forms.id
            WHERE forms.form_id = ?1
            ORDER BY submissions.created_at DESC;
            ",
            form_id,
        )?;

        #[derive(Debug, Deserialize)]
        struct Row {
            encrypted_body: EncryptedSubmissionBody,
            created_at: String,
        }

        stmt.all()
            .await?
            .results::<Row>()?
            .into_iter()
            .map(|row| {
                Ok(Submission {
                    encrypted_body: row.encrypted_body,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.created_at,
                        SQLITE_DATETIME_FORMAT,
                    )?
                    .and_utc(),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }

    #[worker::send]
    pub async fn put_submission(
        &self,
        form_id: FormId,
        submission_id: SubmissionId,
        encrypted_submission: EncryptedSubmissionBody,
    ) -> anyhow::Result<bool> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO submissions (form, submission_id, encrypted_body)
            SELECT forms.id, ?1, ?2
            FROM forms
            WHERE forms.form_id = ?3;
            ",
            submission_id,
            encrypted_submission,
            form_id,
        )?;

        let meta = stmt.run().await?.meta()?;

        if let Some(meta) = meta {
            Ok(meta.changed_db.unwrap_or(false))
        } else {
            Ok(false)
        }
    }

    #[worker::send]
    pub async fn get_client_keys(
        &self,
        form_id: FormId,
        key_id: ClientKeyId,
    ) -> anyhow::Result<Option<ClientKeys>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                keys.public_wrapping_key,
                keys.wrapped_private_key,
                keys.encrypted_comment,
                keys.created_at
            FROM keys
            JOIN forms ON keys.form = forms.id
            WHERE forms.form_id = ?1 AND keys.key_index = ?2;
            ",
            form_id,
            key_id,
        )?;

        #[derive(Debug, Deserialize)]
        struct Row {
            public_signing_key: PublicSigningKey,
            wrapped_private_primary_key: WrappedPrivatePrimaryKey,
            encrypted_comment: EncryptedKeyComment,
            created_at: String,
        }

        let row = stmt.first::<Row>(None).await?;

        row.map(|row| -> anyhow::Result<_> {
            Ok(ClientKeys {
                id: key_id,
                public_signing_key: row.public_signing_key,
                wrapped_private_primary_key: row.wrapped_private_primary_key,
                encrypted_comment: row.encrypted_comment,
                created_at: NaiveDateTime::parse_from_str(&row.created_at, SQLITE_DATETIME_FORMAT)?
                    .and_utc(),
            })
        })
        .transpose()
    }

    #[worker::send]
    pub async fn list_client_keys(&self, form_id: FormId) -> anyhow::Result<Vec<ClientKeys>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                keys.key_index,
                keys.public_signing_key,
                keys.wrapped_private_primary_key,
                keys.encrypted_comment,
                keys.created_at
            FROM keys
            JOIN forms ON keys.form = forms.id
            WHERE forms.form_id = ?1
            ORDER BY keys.key_index ASC;
            ",
            form_id,
        )?;

        #[derive(Debug, Deserialize)]
        struct Row {
            key_index: ClientKeyId,
            public_signing_key: PublicSigningKey,
            wrapped_private_primary_key: WrappedPrivatePrimaryKey,
            encrypted_comment: EncryptedKeyComment,
            created_at: String,
        }

        let rows = stmt.all().await?.results::<Row>()?;

        rows.into_iter()
            .map(|row| {
                Ok(ClientKeys {
                    id: row.key_index,
                    public_signing_key: row.public_signing_key,
                    wrapped_private_primary_key: row.wrapped_private_primary_key,
                    encrypted_comment: row.encrypted_comment,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.created_at,
                        SQLITE_DATETIME_FORMAT,
                    )?
                    .and_utc(),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }

    #[worker::send]
    pub async fn store_client_keys(
        &self,
        form_id: FormId,
        public_signing_key: PublicSigningKey,
        wrapped_private_primary_key: WrappedPrivatePrimaryKey,
        encrypted_comment: EncryptedKeyComment,
    ) -> anyhow::Result<Option<ClientKeyId>> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO keys (
                form,
                key_index,
                public_signing_key,
                wrapped_private_primary_key,
                encrypted_comment
            )
            SELECT
                forms.id,
                COALESCE(
                    (
                        SELECT MAX(keys.key_index) + 1
                        FROM keys
                        JOIN forms ON keys.form = forms.id
                        WHERE forms.form_id = ?1
                        GROUP BY forms.id
                    ),
                    0
                ),
                ?2,
                ?3,
                ?4
            FROM forms
            WHERE forms.form_id = ?1
            RETURNING keys.key_index;
            ",
            form_id,
            public_signing_key,
            wrapped_private_primary_key,
            encrypted_comment,
        )?;

        Ok(stmt.first::<ClientKeyId>(Some("key_index")).await?)
    }

    #[worker::send]
    pub async fn delete_client_key(
        &self,
        form_id: FormId,
        key_id: ClientKeyId,
    ) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            DELETE FROM keys
            WHERE keys.id IN (
                SELECT keys.id
                FROM keys
                JOIN forms ON keys.form = forms.id
                WHERE forms.form_id = ?1 AND keys.key_index = ?2
            );
            ",
            form_id,
            key_id,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn store_ephemeral_server_key(
        &self,
        key_id: ServerKeyId,
        key: EphemeralServerKey,
    ) -> anyhow::Result<()> {
        todo!()
    }

    #[worker::send]
    pub async fn get_ephemeral_server_key(
        &self,
        key_id: ServerKeyId,
    ) -> anyhow::Result<EphemeralServerKey> {
        todo!()
    }

    #[worker::send]
    pub async fn delete_ephemeral_server_key(&self, key_id: ServerKeyId) -> anyhow::Result<()> {
        todo!()
    }
}
