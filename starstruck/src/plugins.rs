pub trait EnginePlugin {
    fn plugin_callback(&self) -> Self {}
}

pub trait EngineObject {
    fn object_callback(&self) -> Self {}
}
