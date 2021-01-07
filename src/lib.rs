use colored::*;
use std::io;

pub mod traits;

pub use crate::traits::*;
use std::io::{stdout, Write};

pub struct CommandsRegister {
    commands: Vec<Box<dyn Command>>,
}

impl CommandsRegister {
    pub fn new() -> CommandsRegister {
        CommandsRegister {commands: vec![]}
    }

    fn get_command_from_command_name(&self, command_name: &str) -> Option<&Box<dyn Command>> {
        let mut command_return = None;
        for command in &self.commands {
           if command.get_command_name() == command_name {
               command_return = Some(command);
           }
        }

        command_return
    }

    fn get_command_from_command_alias(&self, alias: &str) -> Option<&Box<dyn Command>> {
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

        let command_or_alias = input.next()
            .expect("Input is void");

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

    pub fn update(&self) {
        let mut input = String::new();
        print!("{}", self.prompt);
        stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.as_str() == "" {
            return;
        }
        self.commands_register.check_input(input);
    }
}

#[cfg(test)]
mod tests {
}