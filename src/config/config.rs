use std::path::PathBuf;

use uuid::Uuid;

use crate::config::version::{MinecraftType, MinecraftVersion};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub uuid: Uuid,
    pub core_path: PathBuf,
    pub jar_path: PathBuf,
    pub mc_version: MinecraftVersion,
    pub mc_type: MinecraftType,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            core_path: PathBuf::new(),
            jar_path: PathBuf::new(),
            mc_version: MinecraftVersion::Unknown,
            mc_type: MinecraftType::Unknown,
        }
    }
}
