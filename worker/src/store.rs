use std::fmt;

use chrono::NaiveDateTime;
use serde::Deserialize;
use worker::{d1::D1Database, query, D1Result};

use crate::models::{
    EncryptedSubmissionBody, FormId, FormTemplate, KeyIndex, KeyMetadata, Submission, SubmissionId,
    WrappedPrivateKey,
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
            WHERE forms.form_id = ?
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
            SELECT forms.id, ?, ?
            FROM forms
            WHERE forms.form_id = ?;
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
    pub async fn get_form(&self, form_id: FormId) -> anyhow::Result<Option<FormTemplate>> {
        let stmt = query!(
            &self.db,
            "
            SELECT template
            FROM forms
            WHERE form_id = ?;
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
    pub async fn put_form(&self, form_id: FormId, template: FormTemplate) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO forms (form_id, template)
            VALUES (?, ?);
            ",
            form_id,
            serde_json::to_string(&template)?,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_form_and_submissons(&self, form_id: FormId) -> anyhow::Result<()> {
        let delete_submissions_stmt = query!(
            &self.db,
            "
            DELETE FROM submissions
            WHERE submissions.id IN (
                SELECT submissions.id
                FROM submissions
                JOIN forms ON submissions.form = forms.id
                WHERE forms.form_id = ?
            );
            ",
            form_id,
        )?;

        let delete_keys_stmt = query!(
            &self.db,
            "
            DELETE FROM keys
            WHERE keys.id IN (
                SELECT keys.id
                FROM keys
                JOIN forms ON keys.form = forms.id
                WHERE forms.form_id = ?
            );
            ",
            form_id,
        )?;

        let delete_form_stmt = query!(
            &self.db,
            "
            DELETE FROM forms
            WHERE forms.form_id = ?;
            ",
            form_id,
        )?;

        // These queries should be batched so they happen in a single atomic transaction.
        self.db
            .batch(vec![
                delete_submissions_stmt,
                delete_keys_stmt,
                delete_form_stmt,
            ])
            .await?
            .iter()
            .map(D1Result::meta)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    #[worker::send]
    pub async fn get_key(
        &self,
        form_id: FormId,
        index: KeyIndex,
    ) -> anyhow::Result<Option<WrappedPrivateKey>> {
        let stmt = query!(
            &self.db,
            "
            SELECT keys.key AS key
            FROM keys
            JOIN forms ON keys.form = forms.id
            WHERE forms.form_id = ? AND keys.key_index = ?;
            ",
            form_id,
            index,
        )?;

        Ok(stmt.first::<WrappedPrivateKey>(Some("key")).await?)
    }

    #[worker::send]
    pub async fn list_keys(&self, form_id: FormId) -> anyhow::Result<Vec<KeyMetadata>> {
        let stmt = query!(
            &self.db,
            "
            SELECT keys.key_index AS key_index, keys.comment AS comment
            FROM keys
            JOIN forms ON keys.form = forms.id
            WHERE forms.form_id = ?
            ORDER BY keys.key_index ASC;
            ",
            form_id,
        )?;

        Ok(stmt.all().await?.results::<KeyMetadata>()?)
    }

    #[worker::send]
    pub async fn add_key(
        &self,
        form_id: FormId,
        key: WrappedPrivateKey,
        comment: &str,
    ) -> anyhow::Result<Option<KeyIndex>> {
        let stmt = query!(
            &self.db,
            "
            INSERT INTO keys (form, key_index, key, comment)
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
                ?3
            FROM forms
            WHERE forms.form_id = ?1
            RETURNING key_index;
            ",
            form_id,
            key,
            comment,
        )?;

        Ok(stmt.first::<KeyIndex>(Some("key_index")).await?)
    }

    #[worker::send]
    pub async fn delete_key(&self, form_id: FormId, index: KeyIndex) -> anyhow::Result<()> {
        let stmt = query!(
            &self.db,
            "
            DELETE FROM keys
            WHERE keys.id IN (
                SELECT keys.id
                FROM keys
                JOIN forms ON keys.form = forms.id
                WHERE forms.form_id = ? AND keys.key_index = ?
            );
            ",
            form_id,
            index,
        )?;

        stmt.run().await?.meta()?;

        Ok(())
    }
}
