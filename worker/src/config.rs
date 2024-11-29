use std::time::Duration;

pub fn origins() -> Vec<String> {
    [
        "https://api.notwithout.help",
        "https://api-dev.notwithout.help",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn access_token_exp() -> Duration {
    Duration::from_secs(60 * 60)
}

pub fn challenge_token_exp() -> Duration {
    Duration::from_secs(60)
}
