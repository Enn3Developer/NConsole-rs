pub trait Command {
    fn get_command_name(&self) -> &str;
    fn get_command_alias(&self) -> Vec<&str>;

    fn get_help(&self) -> &str;

    fn start(&self) {
        unimplemented!();
    }
    fn end(&self) {
        unimplemented!();
    }

    fn on_command(&self, args: Vec<&str>);
}

pub trait ErrorHandler {
    fn input_void(&self);
    fn wrong_command(&self, command: &str);
}
