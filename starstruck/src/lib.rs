pub mod error;
pub mod prelude;
pub mod vulkan_start;
pub mod plugins;

use std::ptr::null_mut;
use plugins::EnginePlugin;
use vulkan_sys::VkInstance;

use crate::error::EngineError;

pub struct Engine {
    instance: VkInstance,

    plugins: Vec<&dyn EnginePlugin>
}

impl Engine {
    pub fn new(callback: CallbackType) -> Self {
        Self {
            callback,

            instance: null_mut(),

            plugins: vec![]
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

pub struct EngineBase;

impl EnginePlugin for EngineBase {
    fn plugin_make(engine: &mut Engine) -> Self {
        
    }
}