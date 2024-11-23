use std::{collections::HashMap, fmt};

use worker::{d1::D1Database, D1Result};

use crate::models::{EncryptedSubmission, FormId, FormTemplate, SubmissionId};

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
    pub async fn list_submissions(
        &self,
        form_id: FormId,
    ) -> worker::Result<HashMap<SubmissionId, EncryptedSubmission>> {
        let stmt = self.db.prepare(
            "
            SELECT (submission_id, encrypted_body)
            FROM submissions
            JOIN forms ON submissions.form = forms.id
            WHERE forms.from_id = ?;
            ",
        );

        Ok(stmt
            .bind(&[form_id.into()])?
            .all()
            .await?
            .results::<(SubmissionId, EncryptedSubmission)>()?
            .into_iter()
            .collect())
    }

    #[worker::send]
    pub async fn put_submission(
        &self,
        form_id: FormId,
        submission_id: SubmissionId,
        encrypted_submission: EncryptedSubmission,
    ) -> worker::Result<()> {
        let stmt = self.db.prepare(
            "
            INSERT INTO submissions (form, submission_id, encrypted_body)
            SELECT forms.id, ?, ?
            FROM forms
            WHERE forms.form_id = ?;
            ",
        );

        stmt.bind(&[
            submission_id.into(),
            encrypted_submission.into(),
            form_id.into(),
        ])?
        .run()
        .await?
        .meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn get_form(&self, form_id: FormId) -> worker::Result<Option<FormTemplate>> {
        let stmt = self.db.prepare(
            "
            SELECT (template)
            FROM forms
            WHERE form_id = ?;
            ",
        );

        Ok(stmt
            .bind(&[form_id.into()])?
            .first::<String>(Some("template"))
            .await?
            .map(|raw| serde_json::from_str(&raw))
            .transpose()?)
    }

    #[worker::send]
    pub async fn put_form(&self, form_id: FormId, template: FormTemplate) -> worker::Result<()> {
        let stmt = self.db.prepare(
            "
            INSERT INTO forms (form_id, template)
            VALUES (?, ?);
            ",
        );

        stmt.bind(&[form_id.into(), serde_json::to_string(&template)?.into()])?
            .run()
            .await?
            .meta()?;

        Ok(())
    }

    #[worker::send]
    pub async fn delete_form_and_submissons(&self, form_id: FormId) -> worker::Result<()> {
        let delete_submissions_stmt = self
            .db
            .prepare(
                "
                DELETE FROM submissions
                JOIN forms ON submissions.form = forms.id
                WHERE forms.form_id = ?;
                ",
            )
            .bind(&[form_id.clone().into()])?;

        let delete_form_stmt = self
            .db
            .prepare(
                "
                DELETE FROM forms
                WHERE forms.form_id = ?;
                ",
            )
            .bind(&[form_id.into()])?;

        self.db
            .batch(vec![delete_submissions_stmt, delete_form_stmt])
            .await?
            .iter()
            .map(D1Result::meta)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}
