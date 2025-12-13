#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Crashed,
    Killing,
    Killed,
}
