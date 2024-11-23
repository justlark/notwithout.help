use std::{collections::HashMap, fmt};

use worker::d1::D1Database;

use crate::models::{EncryptedSubmission, FormId, SubmissionId};

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
}
