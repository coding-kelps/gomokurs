use config::{ConfigBuilder, ConfigError, File, FileFormat};
use config::builder::DefaultState;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerConfiguration {
    pub protocol: ProtocolConfiguration,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")] 
pub enum ProtocolConfiguration {
    Stdio(StdioConfiguration),
    Tcp(TcpConfiguration),
}

#[derive(Debug, Deserialize, Clone)]
pub struct StdioConfiguration {
    pub binary: PathBuf,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")] 
pub enum TcpConfiguration {
    Active(TcpActiveConfiguration),
    Passive(TcpPassiveConfiguration),
}

#[derive(Debug, Deserialize, Clone)]
pub struct TcpActiveConfiguration {
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TcpPassiveConfiguration {
    pub address: String,
}

impl PlayerConfiguration {
    pub fn new(path: &Path) -> Result<Self, ConfigError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| ConfigError::Message("Invalid UTF-8 in path".into()))?;

        let builder: ConfigBuilder<DefaultState> = ConfigBuilder::default();

        let builder = builder
                .add_source(File::new(path_str, FileFormat::Yaml));
        
        let config = builder.build()?;
        
        config.try_deserialize()
    }
}
