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

pub fn max_request_body_len() -> usize {
    // The longest request bodies we expect to see are form templates and user submissions, which
    // are both just plain text. This should give users enough space to say what they want to say
    // while protecting us from someone uploading the complete works of Shakespeare.
    1024 * 1024
}
