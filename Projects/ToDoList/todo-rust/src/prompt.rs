use std::{collections::HashMap, process::Command};

use colored::Colorize;

use crate::{
  action_manager::{ActionArgs, ActionManager},
  display::DisplayManager,
  reader::Reader,
};

type CommandHandler = fn(&mut Prompt);
pub struct Prompt {
  display: Box<dyn DisplayManager>,
  reader: Box<dyn Reader>,
  commands: HashMap<&'static str, CommandHandler>,
  action_manager: ActionManager,
  run: bool,
  modifications: bool,
}

enum Style {
  Error,
  Success,
  Fancy,
  Default,
}

impl Prompt {
  pub fn new(display: Box<dyn DisplayManager>, reader: Box<dyn Reader>) -> Self {
    Self {
      display,
      reader,
      commands: Self::commands_mapper(),
      action_manager: ActionManager::new(),
      run: false,
      modifications: false,
    }
  }

  fn commands_mapper() -> HashMap<&'static str, CommandHandler> {
    let mut commands: HashMap<&str, CommandHandler> = HashMap::new();
    commands.insert("add", Prompt::command_add);
    commands.insert("display", Prompt::command_display);
    commands.insert("remove", Prompt::command_remove);
    commands.insert("update", Prompt::command_update);
    commands.insert("complete", Prompt::command_complete);
    commands.insert("save", Prompt::command_save);
    commands.insert("exit", Prompt::command_exit);

    commands
  }

  pub fn run(&mut self) {
    self.run = true;
    while self.run {
      self.show();
      let command = self.read_command();
      match self.process_command(command.as_str()) {
        Ok(_) => {}
        Err(e) => {
          self.print(e, Style::Error);
        }
      }
    }
  }

  pub fn show(&mut self) {
    self.print("message", Style::Fancy);
  }
  fn read(&mut self) -> String {
    self.reader.read()
  }

  fn read_command(&mut self) -> String {
    self.read().to_lowercase()
  }

  fn process_command(&mut self, command: &str) -> Result<(), &'static str> {
    match self.commands.get(command) {
      Some(f) => {
        f(self);
        Ok(())
      }
      None => Err("Command not found"),
    }
  }

  fn command_add(&mut self) {
    let mut args = self.get_args();
    if self.wanna_process("You are about to add a new task. Do you want to continue? (y/n)") {
      args.command = Some("add".to_string());
      if self.action_manager.process(args, &*self.display) {
        self.modifications = true;
        self.print("New task added successfully", Style::Success);
      }
    }
  }

  fn wanna_process(&mut self, message: &str) -> bool {
    self.print(message, Style::Fancy);
    let command = self.read_command();
    command == "y"
  }

  fn get_args(&mut self) -> ActionArgs {
    self.print("Type the task name", Style::Default);
    let name = self.read();

    self.print("Type the task description", Style::Default);
    let description = self.read();

    ActionArgs {
      command: None,
      first: Some(name),
      second: Some(description),
      third: None,
    }
  }

  fn command_display(&mut self) {
    self.action_manager.process("display");
  }

  fn command_remove(&mut self) {
    self.action_manager.process("remove");
    self.modifications = true;
  }
  fn command_update(&mut self) {
    self.action_manager.process("update");
    self.modifications = true;
  }
  fn command_complete(&mut self) {
    self.action_manager.process("complete");
    self.modifications = true;
  }
  fn command_save(&mut self) {
    self.action_manager.process("save");
    self.modifications = false;
  }
  fn command_exit(&mut self) {
    if self.modifications {
      self
        .display
        .show("You have unsaved modifications. Do you want to exit? (y/n)".to_string());
      let command = self.read_command();
      if command == "y" {
        self.run = false;
      }
    } else {
      self.run = false;
    }
  }

  fn print(&mut self, message: &str, style: Style) {
    let message = match style {
      Style::Error => message.red(),
      Style::Success => message.green(),
      Style::Fancy => message.cyan(),
      Style::Default => message.white(),
    };
    self.display.show(message.to_string());
  }
}
