use starstruck::prelude::*;

fn main() {
    let engine = Engine::new(callback);
    engine.start().run();
}
