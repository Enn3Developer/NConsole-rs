pub trait Command {
    fn get_command_name(&self) -> &str;
    fn get_command_alias(&self) -> Vec<&str>;

    fn start(&self);
    fn end(&self);

    fn on_command(&self, args: Vec<&str>);
}

pub trait ErrorHandler {
    fn input_void(&self);
    fn wrong_command(&self, command: &str);
}