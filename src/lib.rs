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

pub struct CommandsRegister {
    commands: Vec<Box<dyn Command>>,
    error_handler: Box<dyn ErrorHandler>,
}

impl CommandsRegister {
    pub fn new() -> CommandsRegister {
        CommandsRegister {commands: vec![], error_handler: Box::new(NErrorHandler::new()) }
    }

    pub fn get_error_handler(&self) -> &dyn ErrorHandler {
        self.error_handler.as_ref()
    }

    pub fn set_error_handler(&mut self, error_handler: impl ErrorHandler + 'static) {
        self.error_handler = Box::new(error_handler);
    }

    pub fn get_command_from_command_name(&self, command_name: &str) -> Option<&Box<dyn Command>> {
        let mut command_return = None;
        for command in &self.commands {
           if command.get_command_name() == command_name {
               command_return = Some(command);
           }
        }

        command_return
    }

    pub fn get_command_from_command_alias(&self, alias: &str) -> Option<&Box<dyn Command>> {
        let mut command_return = None;
        for command in &self.commands {
            if command.get_command_alias().contains(&alias) {
                command_return = Some(command);
            }
        }

        command_return
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

    pub fn check_input(&self, mut input: String) -> bool {
        let mut handled = false;

        input = input.to_lowercase();
        let mut input = input.split_whitespace();

        let command_or_alias = input.next().unwrap();

        let mut args: Vec<&str> = vec![];
        for arg in input {
            args.push(arg);
        }

        if let Some(command) = self.get_command_from_command_name(command_or_alias) {
            command.on_command(args);
            handled = true;
        }
        else {
            if let Some(command) = self.get_command_from_command_alias(command_or_alias) {
                command.on_command(args);
                handled = true;
            }
        }

        handled
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
        Console { prompt, commands_register }
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
            self.commands_register.get_error_handler().wrong_command(command);
        }
    }
}

#[cfg(test)]
mod tests {
}