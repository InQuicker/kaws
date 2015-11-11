pub struct Logger {
    enabled: bool,
}

impl Logger {
    pub fn new(enabled: bool) -> Self {
        Logger {
            enabled: enabled,
        }
    }

    pub fn action<'a, F, T>(&self, description: &'a str, action: F) -> T where F: Fn() -> T {
        if self.enabled {
            print!("{}...", description);
        }

        let result = action();

        if self.enabled {
            println!("done.");
        }

        result
    }
}

