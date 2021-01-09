use colored::*;
use std::io;
use std::io::{stdout, Write};

pub mod traits;

pub use crate::traits::*;

struct NErrorHandler;

impl NErrorHandler {
    fn new() -> Self {
        NErrorHandler
    }
}

impl ErrorHandler for NErrorHandler {
    fn input_void(&self) {
        unimplemented!()
    }

    fn wrong_command(&self, command: &str) {
        Console::log(LogTypes::ERR, format!("{}: not a valid command", command));
    }
}

pub struct NHelpCommand<'a> {
    commands: &'a Vec<Box<dyn Command>>,
}

impl NHelpCommand<'_> {
    pub fn new(commands_register: &'static CommandsRegister) -> Self {
        NHelpCommand {
            commands: &commands_register.commands,
        }
    }

    fn get_command(&self, command_str: &str) -> Option<&Box<dyn Command>> {
        for command in self.commands {
            if command.get_command_name() == command_str {
                return Some(command);
            }
            for alias in command.get_command_alias() {
                if alias == command_str {
                    return Some(command);
                }
            }
        }

        None
    }
}

impl Command for NHelpCommand<'_> {
    fn get_command_name(&self) -> &str {
        "help"
    }

    fn get_command_alias(&self) -> Vec<&str> {
        vec!["h", "?"]
    }

    fn get_help(&self) -> &str {
        "Get help for all commands:\
How to use: `help [command]`\
Aliases: `h` and `?`"
    }

    fn on_command(&self, args: Vec<&str>) {
        if args.len() < 1 {
            let mut content = String::new();
            for command in self.commands {
                content.push_str(command.get_command_name());
                content.push('\n');
                content.push_str(command.get_help());
                content.push_str("\n\n");
            }
            Console::print(content);
        } else if args.len() == 1 {
            let command: &Box<dyn Command>;
            if let Some(c) = self.get_command(args[0]) {
                command = c;
            } else {
                return;
            }
            let mut content = String::new();
            content.push_str(command.get_command_name());
            content.push('\n');
            content.push_str(command.get_help());
            content.push_str("\n\n");
            Console::print(content);
        }
    }
}

pub struct CommandsRegister {
    commands: Vec<Box<dyn Command>>,
    error_handler: Box<dyn ErrorHandler>,
}

impl CommandsRegister {
    pub fn new() -> CommandsRegister {
        CommandsRegister {
            commands: vec![],
            error_handler: Box::new(NErrorHandler::new()),
        }
    }

    pub fn get_error_handler(&self) -> &dyn ErrorHandler {
        self.error_handler.as_ref()
    }

    pub fn set_error_handler(&mut self, error_handler: impl ErrorHandler + 'static) {
        self.error_handler = Box::new(error_handler);
    }

    pub fn get_command(&self, command_str: &str) -> Option<&Box<dyn Command>> {
        for command in &self.commands {
            if command.get_command_name() == command_str {
                return Some(command);
            }
            for alias in command.get_command_alias() {
                if alias == command_str {
                    return Some(command);
                }
            }
        }

        None
    }

    pub fn start(&self) {
        self.commands.iter().for_each(|command| {
            command.start();
        });
    }

    pub fn end(&self) {
        self.commands.iter().for_each(|command| {
            command.end();
        });
    }

    pub fn register_command(&mut self, command: impl Command + 'static) {
        self.commands.push(Box::new(command));
    }

    pub fn check_input(&self, input: String) -> bool {
        let mut input = input.split_whitespace();
        let command_or_alias = input.next().unwrap();

        let mut args: Vec<&str> = vec![];
        for arg in input {
            args.push(arg);
        }

        if let Some(command) = self.get_command(command_or_alias) {
            command.on_command(args);
            return true;
        }

        false
    }
}

pub enum LogTypes {
    INFO,
    WARN,
    ERR,
}

pub struct Console {
    prompt: String,
    commands_register: CommandsRegister,
}

impl Console {
    pub fn new(prompt: String, commands_register: CommandsRegister) -> Console {
        Console {
            prompt,
            commands_register,
        }
    }

    pub fn log(log_type: LogTypes, message: String) {
        match log_type {
            LogTypes::INFO => println!("{} {}", "[INFO]".green(), message),
            LogTypes::WARN => println!("{} {}", "[WARNING]".yellow(), message),
            LogTypes::ERR => println!("{} {}", "[ERROR]".red(), message),
        }
    }

    pub fn print(message: String) {
        println!("{}", message);
    }

    pub fn update(&self) {
        let mut input = String::new();
        print!("{}", self.prompt);
        stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().is_empty() {
            self.commands_register.get_error_handler().input_void();
            return;
        }
        let copy = input.clone();
        let command = copy.split_ascii_whitespace().next().unwrap();

        let handled = self.commands_register.check_input(input);

        if !handled {
            self.commands_register
                .get_error_handler()
                .wrong_command(command);
        }
    }
}

#[cfg(test)]
mod tests {}
