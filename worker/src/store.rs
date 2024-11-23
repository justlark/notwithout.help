use worker::{self, kv::KvStore};

use crate::models::{EncryptedSubmission, FormId, FormTemplate, SubmissionId};

fn form_key(form_id: FormId) -> String {
    format!("form:{}", form_id)
}

fn submission_key(form_id: FormId, submission_id: SubmissionId) -> String {
    format!("submission:{}:{}", form_id, submission_id)
}

fn submissions_list_key(form_id: FormId) -> String {
    format!("submission:{}:", form_id)
}

#[worker::send]
pub async fn get_form(kv: &KvStore, form_id: FormId) -> worker::Result<Option<FormTemplate>> {
    Ok(kv.get(&form_key(form_id)).json().await?)
}

#[worker::send]
pub async fn put_form(kv: &KvStore, form_id: FormId, form: &FormTemplate) -> worker::Result<()> {
    Ok(kv.put(&form_key(form_id), form)?.execute().await?)
}

#[worker::send]
pub async fn get_submission(
    kv: &KvStore,
    form_id: FormId,
    submission_id: SubmissionId,
) -> worker::Result<Option<EncryptedSubmission>> {
    Ok(kv
        .get(&submission_key(form_id, submission_id))
        .json()
        .await?)
}

#[worker::send]
pub async fn list_submissions(kv: &KvStore, form_id: FormId) -> worker::Result<Vec<SubmissionId>> {
    let prefix = submissions_list_key(form_id);
    let mut submissions = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut opts = kv.list().prefix(prefix.clone());

        if let Some(cur) = cursor {
            opts = opts.cursor(cur);
        }

        let resp = opts.execute().await?;
        submissions.extend(resp.keys.into_iter().map(|k| {
            k.name
                .strip_prefix(&prefix)
                .expect("unexpected key name")
                .into()
        }));

        match resp.cursor {
            Some(cur) if !resp.list_complete => {
                cursor = Some(cur);
            }
            _ => return Ok(submissions),
        }
    }
}

#[worker::send]
pub async fn put_submission(
    kv: &KvStore,
    form_id: FormId,
    submission_id: SubmissionId,
    submission: EncryptedSubmission,
) -> worker::Result<()> {
    Ok(kv
        .put(&submission_key(form_id, submission_id), submission)?
        .execute()
        .await?)
}
