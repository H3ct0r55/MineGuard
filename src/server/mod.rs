use tokio::sync::RwLock;

use crate::instance::handle::InstanceHandle;

#[derive(Debug)]
pub struct MineGuardServer {
    handle: RwLock<InstanceHandle>,
}

impl MineGuardServer {
    pub fn create() -> Self {
        let new_instance = InstanceHandle::new();
        Self { handle: RwLock::new(new_instance) }
    }
}
