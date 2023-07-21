pub struct StubMe;

impl StubMe {
    pub fn new() -> Self {
        Self
    }

    pub fn do_something(&self) -> i32 {
        // Do something
        12
    }
}