use std::time::Duration;

pub fn current_origin() -> String {
    String::from("https://api.notwithout.help")
}

pub fn allowed_origins() -> Vec<String> {
    vec![current_origin()]
}

pub fn access_token_exp() -> Duration {
    Duration::from_secs(60 * 60)
}

pub fn challenge_token_exp() -> Duration {
    Duration::from_secs(60)
}
