use figment::Figment;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct PulverizerConfig {
    pub db_path: String 
}

pub async fn load_config() -> Result<PulverizerConfig, figment::Error> {
    Ok(Figment::new().merge(figment::providers::Env::prefixed("TP_")).extract()?)
}


