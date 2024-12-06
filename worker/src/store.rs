use std::fmt;

use chrono::NaiveDateTime;
use secrecy::ExposeSecret;
use serde::Deserialize;
use worker::{d1::D1Database, kv::KvStore, query};

use crate::{
    config,
    keys::{EphemeralServerKey, PublicPrimaryKey, PublicSigningKey, WrappedPrivatePrimaryKey},
    models::{
        ChallengeId, ClientKeyId, ClientKeys, EncryptedKeyComment, EncryptedSubmissionBody, FormId,
        FormTemplate, ServerKeyId, Submission, SubmissionId,
    },
};

// SQLite natively understands datetime strings with this format; it uses the format when
// automatically generating timestamps with `DEFAULT CURRENT_TIMESTAMP`.
const SQLITE_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn server_key_ttl() -> u64 {
    config::access_token_exp().as_secs() * 2
}

fn challenge_ttl() -> u64 {
    config::challenge_token_exp().as_secs() * 2
}

fn server_key_key(server_id: &ServerKeyId) -> String {
    format!("key:{}", server_id)
}

fn challenge_key(challenge_id: &ChallengeId) -> String {
    format!("challenge:{}", challenge_id)
}

fn wrap_kv_err(err: worker::kv::KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

#[derive(Debug)]
pub struct UnauthenticatedStore(Store);

impl UnauthenticatedStore {
    pub fn new(db: D1Database, kv: KvStore) -> Self {
        Self(Store { db, kv })
    }

    // If we want to access the store without authenticating, we need to be explicit about it.
    pub fn without_authenticating(&self) -> &Store {
        &self.0
    }
}

pub struct Store {
    db: D1Database,
    kv: KvStore,
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store").finish_non_exhaustive()
    }
}

impl Store {
    #[worker::send]
    pub async fn get_form_template(
        &self,
        form_id: &FormId,
    ) -> anyhow::Result<Option<FormTemplate>> {
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
        form_id: &FormId,
        template: &FormTemplate,
        public_primary_key: &PublicPrimaryKey,
    ) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO forms (form_id, template, public_primary_key)
            VALUES (?1, ?2, ?3);
            ",
            form_id,
            serde_json::to_string(&template)?,
            public_primary_key,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_form(&self, form_id: &FormId) -> anyhow::Result<()> {
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
    pub async fn list_submissions(&self, form_id: &FormId) -> anyhow::Result<Vec<Submission>> {
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
        form_id: &FormId,
        submission_id: &SubmissionId,
        encrypted_submission: &EncryptedSubmissionBody,
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
        form_id: &FormId,
        key_id: &ClientKeyId,
    ) -> anyhow::Result<Option<ClientKeys>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                keys.public_signing_key,
                keys.wrapped_private_primary_key,
                keys.encrypted_comment,
                (
                    SELECT MAX(access_log.accessed_at)
                    FROM access_log
                    WHERE access_log.key = keys.id
                    GROUP BY access_log.key
                ) AS accessed_at
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
            wrapped_private_primary_key: Option<WrappedPrivatePrimaryKey>,
            encrypted_comment: EncryptedKeyComment,
        }

        let row = stmt.first::<Row>(None).await?;

        row.map(|row| -> anyhow::Result<_> {
            Ok(ClientKeys {
                id: key_id.to_owned(),
                public_signing_key: row.public_signing_key,
                wrapped_private_primary_key: row.wrapped_private_primary_key,
                encrypted_comment: row.encrypted_comment,
                accessed_at: None,
            })
        })
        .transpose()
    }

    #[worker::send]
    pub async fn list_client_keys(&self, form_id: &FormId) -> anyhow::Result<Vec<ClientKeys>> {
        let stmt = query!(
            &self.db,
            "
            SELECT
                keys.key_index,
                keys.public_signing_key,
                keys.wrapped_private_primary_key,
                keys.encrypted_comment,
                (
                    SELECT MAX(access_log.accessed_at)
                    FROM access_log
                    WHERE access_log.key = keys.id
                    GROUP BY access_log.key
                ) AS accessed_at
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
            wrapped_private_primary_key: Option<WrappedPrivatePrimaryKey>,
            encrypted_comment: EncryptedKeyComment,
            accessed_at: Option<String>,
        }

        let rows = stmt.all().await?.results::<Row>()?;

        rows.into_iter()
            .map(|row| {
                Ok(ClientKeys {
                    id: row.key_index,
                    public_signing_key: row.public_signing_key,
                    wrapped_private_primary_key: row.wrapped_private_primary_key,
                    encrypted_comment: row.encrypted_comment,
                    accessed_at: row
                        .accessed_at
                        .map(|s| NaiveDateTime::parse_from_str(&s, SQLITE_DATETIME_FORMAT))
                        .transpose()?
                        .map(|dt| dt.and_utc()),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }

    #[worker::send]
    pub async fn store_client_keys(
        &self,
        form_id: &FormId,
        public_signing_key: &PublicSigningKey,
        wrapped_private_primary_key: Option<&WrappedPrivatePrimaryKey>,
        encrypted_comment: &EncryptedKeyComment,
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
    pub async fn update_client_keys(
        &self,
        form_id: &FormId,
        key_id: &ClientKeyId,
        wrapped_private_primary_key: Option<&WrappedPrivatePrimaryKey>,
        encrypted_comment: Option<&EncryptedKeyComment>,
    ) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            UPDATE keys
            SET
                wrapped_private_primary_key = COALESCE(?3, keys.wrapped_private_primary_key),
                encrypted_comment = COALESCE(?4, keys.encrypted_comment)
            WHERE
                keys.form = (
                    SELECT forms.id
                    FROM forms
                    WHERE forms.form_id = ?1
                )
                AND keys.key_index = ?2;
            ",
            form_id,
            key_id,
            wrapped_private_primary_key,
            encrypted_comment,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_client_keys(
        &self,
        form_id: &FormId,
        key_id: &ClientKeyId,
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
        key_id: &ServerKeyId,
        key: &EphemeralServerKey,
    ) -> anyhow::Result<()> {
        self.kv
            .put(&server_key_key(key_id), key.expose_secret())
            .map_err(wrap_kv_err)?
            .expiration_ttl(server_key_ttl())
            .execute()
            .await
            .map_err(wrap_kv_err)?;

        Ok(())
    }

    #[worker::send]
    pub async fn get_ephemeral_server_key(
        &self,
        key_id: &ServerKeyId,
    ) -> anyhow::Result<Option<EphemeralServerKey>> {
        Ok(self
            .kv
            .get(&server_key_key(key_id))
            .text()
            .await
            .map_err(wrap_kv_err)?
            .map(|s| s.parse())
            .transpose()?)
    }

    #[worker::send]
    pub async fn has_challenge_id(&self, challenge_id: &ChallengeId) -> anyhow::Result<bool> {
        Ok(self
            .kv
            .get(&challenge_key(challenge_id))
            .text()
            .await
            .map_err(wrap_kv_err)?
            .is_some())
    }

    #[worker::send]
    pub async fn store_challenge_id(&self, challenge_id: &ChallengeId) -> anyhow::Result<()> {
        self.kv
            .put(&challenge_key(challenge_id), "")
            .map_err(wrap_kv_err)?
            .expiration_ttl(challenge_ttl())
            .execute()
            .await
            .map_err(wrap_kv_err)?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_challenge_id(&self, challenge_id: &ChallengeId) -> anyhow::Result<()> {
        self.kv
            .delete(&challenge_key(challenge_id))
            .await
            .map_err(wrap_kv_err)?;

        Ok(())
    }

    #[worker::send]
    pub async fn log_access(&self, form_id: &FormId, key_id: &ClientKeyId) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO access_log (key)
            SELECT keys.id
            FROM keys
            JOIN forms ON keys.form = forms.id
            WHERE forms.form_id = ?1 AND keys.key_index = ?2;
            ",
            form_id,
            key_id,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }
}
