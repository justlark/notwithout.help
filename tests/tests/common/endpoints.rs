use reqwest::RequestBuilder;

use crate::http;

pub fn post_form() -> RequestBuilder {
    http::client().post(http::path("/forms"))
}

pub fn get_form(form_id: &str) -> RequestBuilder {
    http::client().get(http::path(&format!("/forms/{}", form_id)))
}

pub fn delete_form(form_id: &str) -> RequestBuilder {
    http::client().delete(http::path(&format!("/forms/{}", form_id)))
}

pub fn post_submission(form_id: &str) -> RequestBuilder {
    http::client().post(http::path(&format!("/submissions/{}", form_id)))
}

pub fn get_submissions(form_id: &str) -> RequestBuilder {
    http::client().get(http::path(&format!("/submissions/{}", form_id)))
}

pub fn get_challenge(form_id: &str, client_key_id: &str) -> RequestBuilder {
    http::client().get(http::path(&format!(
        "/challenges/{}/{}",
        form_id, client_key_id
    )))
}

pub fn post_token() -> RequestBuilder {
    http::client().post(http::path("/tokens"))
}
