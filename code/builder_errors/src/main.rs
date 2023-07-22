use thiserror::Error;

#[derive(Error, Debug)]
enum ThingError {
    #[error("Setting must be between 0 and 10")]
    SettingOutOfRange,
}

type ThingResult<T> = Result<T, ThingError>;

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

    fn do_a(mut self) -> ThingResult<Self> {
        self.do_a = true;
        Ok(self)
    }

    fn do_b(mut self) -> ThingResult<Self> {
        self.do_b = true;
        Ok(self)
    }

    fn with_setting(mut self, setting: usize) -> ThingResult<Self> {
        if setting > 10 {
            Err(ThingError::SettingOutOfRange)
        } else {
            self.setting = setting;
            Ok(self)
        }
    }

    fn with_another_setting(mut self, setting: usize) -> ThingResult<Self> {
        self.another_setting = setting;
        Ok(self)
    }

    fn execute(&self) -> ThingResult<()> {
        if self.do_a {
            println!("Doing A");
        }
        if self.do_b {
            println!("Doing B");
        }
        println!("Setting: {}", self.setting);
        println!("Another Setting: {}", self.another_setting);
        Ok(())
    }
}

fn main() -> ThingResult<()> {
    ThingConfig::new()
        .do_a()?
        .with_setting(3)?
        .execute()?;

    Ok(())
}
