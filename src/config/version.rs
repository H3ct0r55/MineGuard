#[derive(Debug, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, Clone)]
pub enum MinecraftVersion {
    Unknown,
    Release(Version),
    #[cfg(feature = "version-custom")]
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum MinecraftType {
    Unknown,
    #[cfg(feature = "mc-vanilla")]
    Vanilla,
    #[cfg(feature = "mc-paper")]
    Paper,
}
