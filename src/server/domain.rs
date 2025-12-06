use std::{path::PathBuf, str::FromStr};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    config::{MinecraftType, MinecraftVersion, Version},
    error::CreationError,
    instance::InstanceHandle,
};

pub struct MineGuardConfig {
    uuid: Uuid,
    server_dir: PathBuf,
    jar_path: PathBuf,
    mc_version: MinecraftVersion,
    mc_type: MinecraftType,
}

pub struct MineGuardServer {
    handle: RwLock<InstanceHandle>,
    config: RwLock<MineGuardConfig>,
}

impl MineGuardConfig {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            server_dir: PathBuf::new(),
            jar_path: PathBuf::new(),
            mc_version: MinecraftVersion::Release(Version::from_str("0.00.00").unwrap()),
            mc_type: MinecraftType::Vanilla,
        }
    }
}

impl MineGuardServer {
    pub fn create(
        mc_version: MinecraftVersion,
        mc_type: MinecraftType,
        directory: PathBuf,
    ) -> Result<Self, CreationError> {
        if !directory.is_dir() {
            return Err(CreationError::DirectoryError);
        }

        todo!()
    }
}
