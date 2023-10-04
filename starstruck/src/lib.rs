pub mod error;

use crate::error::EngineError;

pub struct Engine {
    callback: fn() -> Result<(), EngineError>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            
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
    pub fn callback(engine: Engine) -> Result<(), EngineError>;
}