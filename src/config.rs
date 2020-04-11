pub struct Config {
   pub debug: bool,
   pub trace: bool,
   pub step_wait: isize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            debug: false,
            trace: false,
            step_wait: -1,
        }
    }

    pub fn with_params(debug: bool, trace: bool, step_wait: isize) -> Config {
        Config {
            debug,
            trace,
            step_wait,
        }
    }
}
