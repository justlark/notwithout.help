use std::{sync::OnceLock, time::Duration};

use worker::Env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerEnv {
    Dev,
    Prod,
}

#[derive(Debug)]
struct Config {
    env: WorkerEnv,
    origin: String,
    access_token_exp: Duration,
    challenge_token_exp: Duration,
    max_request_body_len: usize,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            env: match env.var("ENV")?.to_string().as_str() {
                "dev" => WorkerEnv::Dev,
                "prod" => WorkerEnv::Prod,
                _ => return Err(anyhow::anyhow!("invalid ENV")),
            },
            origin: env.var("ORIGIN")?.to_string(),
            access_token_exp: Duration::from_secs(
                env.var("ACCESS_TOKEN_EXP")?.to_string().parse()?,
            ),
            challenge_token_exp: Duration::from_secs(
                env.var("CHALLENGE_TOKEN_EXP")?.to_string().parse()?,
            ),
            max_request_body_len: env.var("MAX_REQUEST_BODY_LEN")?.to_string().parse()?,
        })
        .ok();

    Ok(())
}

fn get_config() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

// Currently unused.
#[allow(dead_code)]
pub fn env() -> WorkerEnv {
    get_config().env
}

pub fn current_origin() -> String {
    get_config().origin.clone()
}

pub fn allowed_origins() -> Vec<String> {
    vec![current_origin()]
}

pub fn access_token_exp() -> Duration {
    get_config().access_token_exp
}

pub fn challenge_token_exp() -> Duration {
    get_config().challenge_token_exp
}

pub fn max_request_body_len() -> usize {
    get_config().max_request_body_len
}
