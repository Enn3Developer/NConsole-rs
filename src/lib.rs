use crate::command::{Command, GenericCommand};
use crate::error_handler::{ErrorHandler, NErrorHandler};
use colored::Colorize;
use std::io;
use std::io::{stdout, Write};

pub mod command;
pub mod error_handler;

pub fn log(log_type: LogTypes, message: String) {
    match log_type {
        LogTypes::INFO => println!("{} {}", "[INFO]".green(), message),
        LogTypes::WARN => println!("{} {}", "[WARNING]".yellow(), message),
        LogTypes::ERR => println!("{} {}", "[ERROR]".red(), message),
    }
}

pub struct CommandsRegister<E> {
    commands: Vec<GenericCommand>,
    error_handler: E,
    help_command: bool,
}

impl Default for CommandsRegister<NErrorHandler> {
    fn default() -> Self {
        Self::new(NErrorHandler::new(), true)
    }
}

impl<E: ErrorHandler> CommandsRegister<E> {
    pub fn new(error_handle: E, help_command: bool) -> Self {
        Self {
            commands: vec![],
            error_handler: error_handle,
            help_command,
        }
    }

    pub fn get_error_handler(&self) -> &E {
        &self.error_handler
    }

    pub fn get_command(&mut self, command_str: &str) -> Option<&mut GenericCommand> {
        for command in self.commands.iter_mut() {
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

    pub fn start(&mut self) {
        self.commands.iter_mut().for_each(|command| {
            command.start();
        });
    }

    pub fn end(&mut self) {
        self.commands.iter_mut().for_each(|command| {
            command.end();
        });
    }

    pub fn register_command(&mut self, command: GenericCommand) {
        self.commands.push(command);
    }

    pub fn check_input(&mut self, input: String) -> bool {
        let mut input = input.split_whitespace();
        let command_or_alias = input.next().unwrap();

        let mut args: Vec<&str> = vec![];
        for arg in input {
            args.push(arg);
        }

        if let Some(command) = self.get_command(command_or_alias) {
            command.on_command(args);
            true
        } else if self.help_command && Self::is_help_command(command_or_alias) {
            self.help(&args);
            true
        } else {
            false
        }
    }

    pub fn help(&mut self, args: &[&str]) {
        match args.len() {
            0 => {
                let mut content = String::new();
                for command in &self.commands {
                    content.push_str(command.get_command_name());
                    content.push('\n');
                    content.push_str(command.get_help());
                    content.push_str("\n\n");
                }
                println!("{content}");
            }
            1 => {
                let command = if let Some(c) = self.get_command(args[0]) {
                    c
                } else {
                    return;
                };
                let mut content = String::new();
                content.push_str(command.get_command_name());
                content.push('\n');
                content.push_str(command.get_help());
                content.push_str("\n\n");
                println!("{content}");
            }
            _ => {}
        }
    }

    fn is_help_command(command_str: &str) -> bool {
        command_str == "help" || command_str == "h" || command_str == "?"
    }
}

pub enum LogTypes {
    INFO,
    WARN,
    ERR,
}

pub struct Console<E: ErrorHandler> {
    prompt: String,
    commands_register: CommandsRegister<E>,
}

impl<E: ErrorHandler> Console<E> {
    pub fn new(prompt: String, commands_register: CommandsRegister<E>) -> Self {
        Self {
            prompt,
            commands_register,
        }
    }

    pub fn update(&mut self) {
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
