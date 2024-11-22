use worker::{self, kv::KvStore};

use crate::models::{FormId, FormTemplate, SubmissionId};

fn form_key(form_id: FormId) -> String {
    format!("form:{}", form_id)
}

fn submission_key(form_id: FormId, submission_id: SubmissionId) -> String {
    format!("submission:{}:{}", form_id, submission_id)
}

pub async fn get_form(kv: &KvStore, form_id: FormId) -> worker::Result<Option<FormTemplate>> {
    Ok(kv.get(&form_key(form_id)).json().await?)
}
