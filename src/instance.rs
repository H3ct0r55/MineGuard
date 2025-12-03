use std::{path::PathBuf, sync::Arc};

use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    config::{MinecraftType, MinecraftVersion, StreamLine, StreamType},
    error::{HandleError, SubscribeError},
};

#[derive(Debug, Clone)]
pub struct InstanceData {
    pub root_dir: PathBuf,
    pub jar_path: PathBuf,
    pub mc_version: MinecraftVersion,
    pub mc_type: MinecraftType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
}

#[derive(Debug)]
pub struct InstanceHandle {
    pub data: InstanceData,
    pub status: InstanceStatus,
    stdout_tx: Option<broadcast::Sender<StreamLine>>,
    stderr_tx: Option<broadcast::Sender<StreamLine>>,
}

impl InstanceHandle {
    pub fn new_with_params(
        root_dir: &str,
        jar_path: &str,
        mc_version: &str,
        mc_type: MinecraftType,
    ) -> Result<Self, HandleError> {
        let parsed_version: MinecraftVersion = mc_version
            .parse()
            .map_err(|_| HandleError::InvalidVersion(mc_version.to_string()))?;

        let root: PathBuf = root_dir.into();
        if !root.exists() || !root.is_dir() {
            return Err(HandleError::InvalidDirectory(root_dir.to_string()));
        }

        let path: PathBuf = jar_path.into();
        let conc = root.join(path.clone());
        if !path.is_relative() || !conc.is_file() {
            return Err(HandleError::InvalidPathJAR(jar_path.to_string()));
        }

        let data = InstanceData {
            root_dir: root,
            jar_path: path,
            mc_version: parsed_version,
            mc_type: mc_type,
        };

        let status = InstanceStatus::Stopped;
        Ok(Self {
            data,
            status,
            stdout_tx: None,
            stderr_tx: None,
        })
    }

    pub fn subscribe(
        &self,
        stream: StreamType,
    ) -> Result<BroadcastStream<StreamLine>, SubscribeError> {
        match stream {
            StreamType::Stdout => {
                let rx = match &self.stdout_tx {
                    Some(value) => value.subscribe(),
                    None => return Err(SubscribeError::NoStdout),
                };
                Ok(BroadcastStream::new(rx))
            }
            StreamType::Stderr => {
                let rx = match &self.stderr_tx {
                    Some(value) => value.subscribe(),
                    None => return Err(SubscribeError::NoStdout),
                };
                Ok(BroadcastStream::new(rx))
            }
        }
    }
}
