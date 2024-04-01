use crate::{log, LogTypes};

pub trait ErrorHandler {
    fn input_void(&self);
    fn wrong_command(&self, command: &str);
}

#[derive(Default)]
pub struct NErrorHandler;

impl NErrorHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ErrorHandler for NErrorHandler {
    fn input_void(&self) {
        unimplemented!()
    }

    fn wrong_command(&self, command: &str) {
        log(LogTypes::ERR, format!("{}: not a valid command", command));
    }
}
