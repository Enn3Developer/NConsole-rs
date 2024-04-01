pub trait Command {
    fn get_command_name(&self) -> &str;
    fn get_command_alias(&self) -> &[String];

    fn get_help(&self) -> &str;

    fn start(&mut self) {}
    fn end(&mut self) {}

    fn on_command(&mut self, args: Vec<&str>);
}

pub type CommandCallback = Box<dyn FnMut(Vec<&str>)>;

pub struct GenericCommand {
    name: String,
    aliases: Vec<String>,
    help: String,
    on_command_callback: CommandCallback,
}

impl GenericCommand {
    pub fn new<F: FnMut(Vec<&str>) + 'static>(
        name: String,
        aliases: Vec<String>,
        help: String,
        on_command: F,
    ) -> Self {
        Self {
            name,
            aliases,
            help,
            on_command_callback: Box::new(on_command),
        }
    }
}

impl Command for GenericCommand {
    fn get_command_name(&self) -> &str {
        &self.name
    }

    fn get_command_alias(&self) -> &[String] {
        &self.aliases
    }

    fn get_help(&self) -> &str {
        &self.help
    }

    fn on_command(&mut self, args: Vec<&str>) {
        let func = &mut self.on_command_callback;
        func(args);
    }
}
