use std::{process::Stdio, sync::Arc};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    sync::{broadcast, RwLock},
};
use tokio_util::sync::CancellationToken;

use crate::{config::config::ServerConfig, error::HandleError, instance::types::InstanceStatus};

#[derive(Debug)]
pub struct InstanceHandle {
    config: Arc<RwLock<ServerConfig>>,
    status: Arc<RwLock<InstanceStatus>>,
    child: Option<Arc<RwLock<Child>>>,
    shutdown: CancellationToken,
    stdout_tx: broadcast::Sender<String>,
    stderr_tx: broadcast::Sender<String>,
}

impl InstanceHandle {
    /// Create a new `InstanceHandle` with a blank `ServerConfig`
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(ServerConfig::new())),
            status: Arc::new(RwLock::new(InstanceStatus::Stopped)),
            child: None,
            shutdown: CancellationToken::new(),
            stdout_tx: broadcast::Sender::new(1024),
            stderr_tx: broadcast::Sender::new(1024),
        }
    }

    /// Create a new `InstanceHandle` with a `ServerConfig`, config is consumed
    pub fn with_cfg(config: ServerConfig) -> Result<Self, HandleError> {
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(InstanceStatus::Stopped)),
            child: None,
            shutdown: CancellationToken::new(),
            stdout_tx: broadcast::Sender::new(1024),
            stderr_tx: broadcast::Sender::new(1024),
        })
    }

    pub async fn start(&mut self) -> Result<(), HandleError> {
        if !self.stopped_killed_or_crashed().await {
            return Err(HandleError::StartFailedNotStopped);
        }

        if self.child.is_some() {
            return Err(HandleError::StartFailedChildExists);
        }

        _ = self.change_status(InstanceStatus::Starting);

        let mut command = self.build_command().await;
        let child = command.spawn();

        self.setup_std_pumps().await?;

        if cfg!(feature = "event") {
            // TODO: await server Done (...)! before status::Running for event module
            todo!()
        } else {
            _ = self.change_status(InstanceStatus::Running);
        }
        todo!()
    }

    pub async fn stop(&mut self) -> Result<(), HandleError> {
        if self.get_status().await != InstanceStatus::Running {
            return Err(HandleError::StopFailedNotRunning);
        }

        let child = self.child.clone().ok_or(HandleError::StopFailedChildNotExists)?;

        _ = self.change_status(InstanceStatus::Stopping);

        let child_w = child.write().await;

        // TODO:: Create send command for graceful stop, finish logic with shutdown token
        todo!()
    }

    pub async fn kill(&mut self) -> Result<(), HandleError> {
        let child = self.child.clone().ok_or(HandleError::KillFailedChildNotExists)?;

        _ = self.change_status(InstanceStatus::Killing);

        let mut child_w = child.write().await;

        child_w.kill().await.map_err(|_| HandleError::KillFailledInternal)?;

        // TODO:: Finish kill logic including updating shutdown token status
        todo!()
    }
}

// region:  --- Utils

impl InstanceHandle {
    async fn stopped_killed_or_crashed(&self) -> bool {
        let status = self.get_status().await;

        if status == InstanceStatus::Stopped
            || status == InstanceStatus::Killed
            || status == InstanceStatus::Crashed
        {
            return true;
        }
        false
    }

    async fn get_status(&self) -> InstanceStatus {
        let status_r = self.status.read().await;
        let res = status_r.clone();
        drop(status_r);

        res
    }

    async fn change_status(
        &mut self,
        new_status: InstanceStatus,
    ) -> (InstanceStatus, InstanceStatus) {
        let mut status_w = self.status.write().await;
        let old = status_w.clone();
        *status_w = new_status.clone();
        drop(status_w);
        (old, new_status)
    }

    async fn get_config(&self) -> ServerConfig {
        let config_r = self.config.read().await;
        let res = config_r.clone();
        drop(config_r);

        res
    }

    async fn build_command(&self) -> Command {
        let cfg = self.get_config().await;
        let mut command = Command::new("java");

        command.arg("-jar").arg(&cfg.jar_path).arg("nogui").current_dir(&cfg.core_path);

        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        command
    }
}

async fn get_status_arc(arc: Arc<RwLock<InstanceStatus>>) -> InstanceStatus {
    let status_r = arc.read().await;
    let res = status_r.clone();
    drop(status_r);

    res
}

async fn change_status_arc(
    arc: Arc<RwLock<InstanceStatus>>,
    new_status: InstanceStatus,
) -> (InstanceStatus, InstanceStatus) {
    let mut status_w = arc.write().await;
    let old = status_w.clone();
    *status_w = new_status.clone();
    drop(status_w);
    (old, new_status)
}

// endregion:   --- Utils

// region:  --- StdStream
impl InstanceHandle {
    async fn setup_std_pumps(&mut self) -> Result<(), HandleError> {
        let child = self.child.clone().ok_or(HandleError::PumpsFailedNoChild)?;

        let mut child_w = child.write().await;

        let stdout_rx = child_w.stdout.take().ok_or(HandleError::PumpsFailedNoStdout)?;
        let stderr_rx = child_w.stderr.take().ok_or(HandleError::PumpsFailedNoStderr)?;

        let stdout_tx = self.stdout_tx.clone();
        let stderr_tx = self.stderr_tx.clone();

        let status_stdout = self.status.clone();
        let status_stderr = self.status.clone();

        tokio::spawn(async move {
            let mut stdout_br = BufReader::new(stdout_rx).lines();
            loop {
                match stdout_br.next_line().await {
                    Ok(Some(line)) => {
                        _ = stdout_tx.send(line);
                    }
                    _ => {
                        let status = get_status_arc(status_stdout.clone()).await;
                        if status == InstanceStatus::Running {
                            change_status_arc(status_stdout, InstanceStatus::Crashed).await;
                        }
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            let mut stdout_br = BufReader::new(stderr_rx).lines();
            loop {
                match stdout_br.next_line().await {
                    Ok(Some(line)) => {
                        _ = stderr_tx.send(line);
                    }
                    _ => {
                        let status = get_status_arc(status_stderr.clone()).await;
                        if status == InstanceStatus::Running {
                            change_status_arc(status_stderr, InstanceStatus::Crashed).await;
                        }
                        break;
                    }
                }
            }
        });

        todo!()
    }
}
// endregion:   --- StdStream
