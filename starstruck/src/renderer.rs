use std::ptr::null_mut;

pub struct Renderer {
    pub instance: (),
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            instance: ()
        }
    }
}
