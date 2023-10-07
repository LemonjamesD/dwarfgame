pub mod error;
pub mod prelude;
pub mod vulkan_start;
pub mod plugins;

use std::ptr::null_mut;
use plugins::EnginePlugin;
use vk_sys::VkInstance;

use crate::error::EngineError;

pub struct Engine {
    instance: VkInstance,

    plugins: Vec<Box<dyn EnginePlugin>>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            instance: null_mut(),

            plugins: vec![]
        }
    }

    // Does a bunch of start up tasks
    pub fn start(self) -> Self {
        for plugin in self.plugins {
            plugin.plugin_make(&mut self);
        }
        self
    }

    pub fn run(self) -> ! {
        loop {
            for plugin in self.plugins {
                plugin.plugin_run(&mut self)
            }
        }
    }
}

pub struct EngineBase;

impl EnginePlugin for EngineBase {
    fn plugin_make(&self, engine: &mut Engine) {
        
    }
}