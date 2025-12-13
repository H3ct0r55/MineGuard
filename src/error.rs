use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("internal library error, this shouldn't happen")]
    InternalError,

    #[error("failed to start handle, not stopped")]
    StartFailedNotStopped,

    #[error("failed to stop handle, not running")]
    StopFailedNotRunning,

    #[error("failed to start handle, child exists")]
    StartFailedChildExists,
    #[error("failed to stop handle, child doesn't exists")]
    StopFailedChildNotExists,

    #[error("failed to kill handle, child doesn't exists")]
    KillFailedChildNotExists,

    #[error("failed to kill handle, internal error")]
    KillFailledInternal,

    #[error("failed to start pumps, child inexistant")]
    PumpsFailedNoChild,

    #[error("failed to start pumps, no stdout in child")]
    PumpsFailedNoStdout,

    #[error("failed to start pumps, no stderr in child")]
    PumpsFailedNoStderr,
}
