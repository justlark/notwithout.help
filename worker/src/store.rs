use std::fmt;

use chrono::NaiveDateTime;
use serde::Deserialize;
use worker::{d1::D1Database, query};

use crate::{
    keys::{FormId, PublicWrappingKey, SubmissionId},
    models::{
        ClientKeyId, ClientKeyPair, EncryptedKeyComment, EncryptedSubmissionBody, FormTemplate,
        Submission, WrappedPrivateClientKey,
    },
};

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
    pub async fn get_client_keys(
        &self,
        form_id: FormId,
        key_id: ClientKeyId,
    ) -> anyhow::Result<Option<ClientKeyPair>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                client_keys.public_wrapping_key,
                client_keys.wrapped_private_key,
                client_keys.encrypted_comment,
                client_keys.created_at
            FROM client_keys
            JOIN forms ON client_keys.form = forms.id
            WHERE forms.form_id = ?1 AND client_keys.key_index = ?2;
            ",
            form_id,
            key_id,
        )?;

        #[derive(Debug, Deserialize)]
        struct Row {
            public_wrapping_key: PublicWrappingKey,
            wrapped_private_key: WrappedPrivateClientKey,
            encrypted_comment: EncryptedKeyComment,
            created_at: String,
        }

        let row = stmt.first::<Row>(None).await?;

        Ok(row
            .map(|row| -> anyhow::Result<_> {
                Ok(ClientKeyPair {
                    id: key_id,
                    public_wrapping_key: row.public_wrapping_key,
                    wrapped_private_key: row.wrapped_private_key,
                    encrypted_comment: row.encrypted_comment,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.created_at,
                        SQLITE_DATETIME_FORMAT,
                    )?
                    .and_utc(),
                })
            })
            .transpose()?)
    }

    #[worker::send]
    pub async fn list_client_keys(&self, form_id: FormId) -> anyhow::Result<Vec<ClientKeyPair>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                client_keys.key_index,
                client_keys.public_wrapping_key,
                client_keys.wrapped_private_key,
                client_keys.encrypted_comment,
                client_keys.created_at
            FROM client_keys
            JOIN forms ON client_keys.form = forms.id
            WHERE forms.form_id = ?1
            ORDER BY client_keys.key_index ASC;
            ",
            form_id,
        )?;

        #[derive(Debug, Deserialize)]
        struct Row {
            key_index: ClientKeyId,
            public_wrapping_key: PublicWrappingKey,
            wrapped_private_key: WrappedPrivateClientKey,
            encrypted_comment: EncryptedKeyComment,
            created_at: String,
        }

        let rows = stmt.all().await?.results::<Row>()?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Ok(ClientKeyPair {
                    id: row.key_index,
                    public_wrapping_key: row.public_wrapping_key,
                    wrapped_private_key: row.wrapped_private_key,
                    encrypted_comment: row.encrypted_comment,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.created_at,
                        SQLITE_DATETIME_FORMAT,
                    )?
                    .and_utc(),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?)
    }

    #[worker::send]
    pub async fn add_client_key(
        &self,
        form_id: FormId,
        public_wrapping_key: PublicWrappingKey,
        wrapped_private_key: WrappedPrivateClientKey,
        encrypted_comment: EncryptedKeyComment,
    ) -> anyhow::Result<Option<ClientKeyId>> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO client_keys (
                form,
                key_index,
                public_wrapping_key,
                wrapped_private_key,
                encrypted_comment
            )
            SELECT
                forms.id,
                COALESCE(
                    (
                        SELECT MAX(client_keys.key_index) + 1
                        FROM client_keys
                        JOIN forms ON client_keys.form = forms.id
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
            RETURNING client_keys.key_index;
            ",
            form_id,
            public_wrapping_key,
            wrapped_private_key,
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
            DELETE FROM client_keys
            WHERE client_keys.id IN (
                SELECT client_keys.id
                FROM client_keys
                JOIN forms ON client_keys.form = forms.id
                WHERE forms.form_id = ?1 AND client_keys.key_index = ?2
            );
            ",
            form_id,
            key_id,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }
}
