use std::{ops::RangeInclusive, path::PathBuf, str::FromStr};

use tokio::{
    fs::{File, create_dir},
    io::{self, AsyncWriteExt},
    sync::RwLock,
};
use uuid::Uuid;

use crate::{
    config::{MinecraftType, MinecraftVersion, Version},
    error::CreationError,
    instance::InstanceHandle,
    manifests::vanilla::{VanillaManifestV2, VanillaManifestV2Version, VanillaReleaseManifest},
    server,
};

pub struct MineGuardConfig {
    uuid: Uuid,
    server_dir: PathBuf,
    jar_path: PathBuf,
    mc_version: MinecraftVersion,
    mc_type: MinecraftType,
}

pub struct MineGuardServer {
    pub handle: RwLock<InstanceHandle>,
    pub config: RwLock<MineGuardConfig>,
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
    pub async fn create(
        mc_version: MinecraftVersion,
        mc_type: MinecraftType,
        directory: PathBuf,
    ) -> Result<Self, CreationError> {
        if !directory.is_dir() {
            return Err(CreationError::DirectoryError);
        }

        let uuid = Uuid::new_v4();

        let server_root = directory.join(uuid.to_string());
        let jar_path_rel =
            PathBuf::from_str("server.jar").map_err(|_| CreationError::DirectoryError)?;
        let jar_path_full = server_root.join(jar_path_rel.clone());

        create_dir(server_root.clone())
            .await
            .map_err(|_| CreationError::DirectoryError)?;

        let mut url = String::new();

        if mc_type == MinecraftType::Vanilla {
            let vanilla_manifest = VanillaManifestV2::load()
                .await
                .map_err(|_| CreationError::ManifestError)?;

            let find_ver = match vanilla_manifest
                .find(mc_version.clone())
                .map_err(|_| CreationError::ManifestError)?
            {
                Some(val) => val,
                None => return Err(CreationError::VersionError),
            };

            let release_manifest = VanillaReleaseManifest::load(find_ver)
                .await
                .map_err(|_| CreationError::ManifestError)?;

            url = release_manifest.server_url();
        }

        let resp = reqwest::get(url)
            .await
            .map_err(|_| CreationError::NetworkError)?;
        let mut body = resp
            .bytes()
            .await
            .map_err(|_| CreationError::NetworkError)?;
        let mut out = File::create(jar_path_full)
            .await
            .map_err(|_| CreationError::DirectoryError)?;
        out.write_all_buf(&mut body)
            .await
            .map_err(|_| CreationError::DirectoryError)?;

        let config = MineGuardConfig {
            uuid: uuid,
            server_dir: server_root,
            jar_path: jar_path_rel,
            mc_version: mc_version,
            mc_type: mc_type,
        };

        let handle = InstanceHandle::new_with_params(
            config.server_dir.clone(),
            config.jar_path.clone(),
            config.mc_version.clone(),
            config.mc_type.clone(),
        )
        .map_err(|_| CreationError::CreationError)?;

        let server = MineGuardServer {
            config: RwLock::new(config),
            handle: RwLock::new(handle),
        };

        Ok(server)
    }
}
