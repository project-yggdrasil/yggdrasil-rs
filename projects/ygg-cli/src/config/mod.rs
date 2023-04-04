use serde::{Deserialize, Deserializer, Serialize};
use std::{
    env::current_dir,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use yggdrasil_error::YggdrasilError;

mod modes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YccConfig {
    language: SupportedMode,
    export: Vec<String>,
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub enum SupportedMode {
    Rust,
}

impl YccConfig {
    pub fn load_path<P: AsRef<Path>>(config: Option<P>) -> Result<Self, YggdrasilError> {
        let (path, ext) = find_config(config).unwrap_or_else(|e| {
            tracing::error!("{}", e);
            (include_str!("Default.json5").to_string(), "json".to_string())
        });
        let config: Self = match ext.as_str() {
            "json" | "json5" => json5::from_str(&path)?,
            _ => {
                unimplemented!()
            }
        };
        Ok(config)
    }
}

fn find_config<P: AsRef<Path>>(config: Option<P>) -> Result<(String, String), YggdrasilError> {
    let file = match config.or_else(|| find_default()) {
        Some(s) => s.as_ref(),
        None => Err(YggdrasilError::config_error("未找到配置文件 `Yggdrasil.json5`, 启用默认设置"))?,
    };
    let config = read_to_string(file)?;
    let path = match file.extension().and_then(|s| s.to_str()) {
        Some(s) => s.to_string(),
        None => {
            tracing::warn!("ConfigError: 无法识别格式, 假定为 json");
            "json".to_string()
        }
    };
    Ok((config, path))
}

fn find_default() -> Option<PathBuf> {
    println!("workspace {:?}", current_dir());
    for file in current_dir()? {}
}
