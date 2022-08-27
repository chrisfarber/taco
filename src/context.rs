pub struct Context {
    state: i32,
}

impl Context {
    pub fn new(state: i32) -> Self {
        Self { state: state }
    }
}
