struct ThingConfig {
    do_a: bool,
    do_b: bool,
    setting: usize,
    another_setting: usize,
}

#[allow(dead_code)]
impl ThingConfig {
    fn new() -> Self {
        ThingConfig {
            do_a: false,
            do_b: false,
            setting: 0,
            another_setting: 0,
        }
    }

    fn do_a(mut self) -> Self {
        self.do_a = true;
        self
    }

    fn do_b(mut self) -> Self {
        self.do_b = true;
        self
    }

    fn with_setting(mut self, setting: usize) -> Self {
        self.setting = setting;
        self
    }

    fn with_another_setting(mut self, setting: usize) -> Self {
        self.another_setting = setting;
        self
    }

    fn execute(&self) {
        if self.do_a {
            println!("Doing A");
        }
        if self.do_b {
            println!("Doing B");
        }
        println!("Setting: {}", self.setting);
        println!("Another Setting: {}", self.another_setting);
    }
}

fn main() {
    ThingConfig::new()
        .do_a()
        .with_setting(3)
        .execute();
}
