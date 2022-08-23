pub trait Command {
    fn get_command_name(&self) -> &str;
    fn get_command_alias(&self) -> Vec<&str>;

    fn get_help(&self) -> &str;

    fn start(&mut self) {
        unimplemented!();
    }
    fn end(&mut self) {
        unimplemented!();
    }

    fn on_command(&mut self, args: Vec<&str>);
}

pub trait ErrorHandler {
    fn input_void(&self);
    fn wrong_command(&self, command: &str);
}
