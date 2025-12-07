#![cfg(feature = "mc-vanilla")]

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use crate::{config::MinecraftVersion, error::ManifestError};

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaManifestV2 {
    latest: VanillaManifestV2Latest,
    versions: Vec<VanillaManifestV2Version>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaManifestV2Latest {
    release: String,
    snapshot: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaManifestV2Version {
    id: String,
    #[serde(rename = "type")]
    mc_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaReleaseManifest {
    downloads: VanillaReleaseManifestDownloads,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VanillaReleaseManifestDownloads {
    client: VanillaReleaseManifestDownloadsItem,
    client_mappings: VanillaReleaseManifestDownloadsItem,
    server: VanillaReleaseManifestDownloadsItem,
    server_mappings: VanillaReleaseManifestDownloadsItem,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VanillaReleaseManifestDownloadsItem {
    sha1: String,
    size: u32,
    url: String,
}

impl VanillaReleaseManifest {
    pub async fn load(version: VanillaManifestV2Version) -> Result<Self, ManifestError> {
        let client = Client::new();

        let manifest: VanillaReleaseManifest = client
            .get(&version.url)
            .send()
            .await
            .map_err(|_| ManifestError::LoadUrlError)?
            .error_for_status()
            .map_err(|_| ManifestError::LoadUrlError)?
            .json()
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                ManifestError::JsonParseError
            })?;

        Ok(manifest)
    }

    pub fn server_url(&self) -> String {
        self.downloads.server.url.clone()
    }
}

impl VanillaManifestV2 {
    pub async fn load() -> Result<Self, ManifestError> {
        let client = Client::new();

        let manifest: VanillaManifestV2 = client
            .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .send()
            .await
            .map_err(|_| ManifestError::LoadUrlError)?
            .error_for_status()
            .map_err(|_| ManifestError::LoadUrlError)?
            .json()
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                ManifestError::JsonParseError
            })?;

        Ok(manifest)
    }

    pub fn find(
        &self,
        version: MinecraftVersion,
    ) -> Result<Option<VanillaManifestV2Version>, ManifestError> {
        let id = version.to_string();

        let found = match self.versions.iter().find(|p| p.id == id) {
            Some(val) => Some(val.clone()),
            None => None,
        };

        Ok(found)
    }
}
