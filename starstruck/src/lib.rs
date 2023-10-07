pub mod error;
pub mod prelude;
pub mod vulkan_instance;
pub mod plugins;
pub mod renderer;
pub mod helpers;

use std::ptr::null_mut;
use plugins::EnginePlugin;

use crate::error::EngineError;
use crate::renderer::Renderer;

pub struct Engine {
    renderer: Renderer,

    plugins: Vec<Box<dyn EnginePlugin>>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),

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