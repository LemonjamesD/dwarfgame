use starstruck::prelude::*;

fn callback() -> Result<(), EngineError> {
    Ok(())
}

fn main() {
    let mut engine = Engine::new(callback);
    engine.start().run();
}
