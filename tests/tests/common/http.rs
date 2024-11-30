use serde_json::json;

const DEFAULT_API_URL: &str = "http://localhost:8787";

pub fn api_url() -> String {
    dotenv::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

pub fn client() -> reqwest::Client {
    reqwest::Client::new()
}
pub fn path(path: &str) -> String {
    format!("{}{}", api_url(), path)
}

pub async fn create_form() -> anyhow::Result<reqwest::Response> {
    Ok(client()
        .post(path("/forms"))
        .json(&json!({
            "public_primary_key": "<public_primary_key>",
            "public_signing_key": "Vp0SD6ySAex2vXtsaA8SbXKS3gS35yWO56MTWk2aJzw=",
            "org_name": "<org_name>",
            "description": "<description>",
            "contact_methods": ["<contact_method>"]
        }))
        .send()
        .await?)
}
