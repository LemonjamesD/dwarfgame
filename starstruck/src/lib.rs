pub mod error;
pub mod prelude;
pub mod vulkan_start;
pub mod plugins;

use std::ptr::null_mut;
use vulkan_sys::VkInstance;

use crate::error::EngineError;

pub type CallbackType = fn() -> Result<(), EngineError>;

pub struct Engine {
    callback: CallbackType,

    instance: VkInstance,
}

impl Engine {
    pub fn new(callback: CallbackType) -> Self {
        Self {
            callback,
            instance: null_mut(),
        }
    }

    // Does a bunch of start up tasks
    pub fn start(self) -> Self {
        self
    }

    pub fn run(self) -> Self {
        self
    }
}

pub trait EngineCallback {
    fn callback(engine: Engine) -> Result<(), EngineError>;
}
