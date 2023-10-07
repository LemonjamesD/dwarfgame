use crate::Engine;

/// A plugin is just a piece of code that runs and interacts with the engine
pub trait EnginePlugin {
    fn plugin_make(engine: &mut Engine) -> Self;
}

/// A physical object in the game that has state that the engine needs to manage
pub trait EngineObject<T: Default> {
    fn object_make(engine: &mut Engine, state: T) -> Self;
}

/// Called when the engine needs to cleanup an object/plugin
pub trait EngineCleanup {
    fn cleanup(&self, engine: &mut Engine);
}