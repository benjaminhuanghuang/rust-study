use std::collections::HashMap;

use colored::Colorize;

use crate::{
  action_manager::{ActionArgs, ActionManager},
  display::DisplayManager,
  menu::{menu_logo, menu_show},
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
  logo: String,
  menu: String,
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
      logo: menu_logo(),
      menu: menu_show(),
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
    let logo = menu_logo();
    let menu = menu_show();

    while self.run {
      self.print(logo.as_str(), Style::Fancy);
      self.print(menu.as_str(), Style::Default);

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

  fn wanna_process(&mut self, message: &'static str) -> bool {
    loop {
      self.print(message, Style::Default);
      let input = self.read();
      match input.as_str() {
        "yes" => return true,
        "no" => {
          self.print("Operation canceled", Style::Error);
          return false;
        }
        _ => self.print("Invalid option", Style::Error),
      }
    }
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
    let args = ActionArgs {
      command: Some("display".to_owned()),
      first: None,
      second: None,
      third: None,
    };
    self.action_manager.process(args, &*self.display);
  }

  fn command_remove(&mut self) {
    match self.ask_id("Type the task id to remove or type 'exit' to cancel: ") {
      Some(id) => {
        let args = ActionArgs {
          command: Some("remove".to_owned()),
          first: Some(id),
          second: None,
          third: None,
        };
        if self.wanna_process("You are about to remove a task. Do you want to continue? (y/n)") {
          if self.action_manager.process(args, &*self.display) {
            self.modifications = true;
            self.print("Task removed successfully", Style::Success);
          } else {
            self.print("Couldn't remove the task.", Style::Error);
          }
        }
      }
      None => (),
    }
  }

  fn ask_id(&mut self, message: &'static str) -> Option<String> {
    loop {
      self.print(message, Style::Default);
      let input = self.read();
      match input.as_str() {
        "exit" => {
          self.print("Operation canceled", Style::Error);
          return None;
        }
        _ => match input.parse::<u32>().is_ok() {
          true => return Some(input),
          false => self.print("Invalid id", Style::Error),
        },
      }
      self.print("Invalid id", Style::Error);
    }
  }
  fn command_update(&mut self) {
    match self.ask_id("Type the task id to update or type 'exit' to cancel: ") {
      Some(id) => {
        let args = self.get_args();
        args.command = Some("update".to_owned());
        args.third = Some(id);
        if self.wanna_process("You are about to update a task. Do you want to continue? (y/n)") {
          if self.action_manager.process(args, &*self.display) {
            self.modifications = true;
            self.print("Task updated successfully", Style::Success);
          } else {
            self.print("Couldn't update the task.", Style::Error);
          }
        }
      }
      None => (),
    }
  }
  fn command_complete(&mut self) {
    match self.ask_id("Type the task id to complete or type 'exit' to cancel: ") {
      Some(id) => {
        let args = self.get_args();
        args.command = Some("complete".to_owned());
        args.third = Some(id);
        if self.wanna_process("You are about to complete a task. Do you want to continue? (y/n)") {
          if self.action_manager.process(args, &*self.display) {
            self.modifications = true;
            self.print("Task complete successfully", Style::Success);
          } else {
            self.print("Couldn't complete the task.", Style::Error);
          }
        }
      }
      None => (),
    }
  }
  fn command_save(&mut self) {
    if self.modifications
      && self.wanna_process("You are about to save the tasks. Do you want to continue? (y/n)")
    {
      let args = ActionArgs {
        command: Some("save".to_owned()),
        first: None,
        second: None,
        third: None,
      };
      self.action_manager.process(args, &*self.display);
      self.modifications = false;
    }
  }
  fn command_exit(&mut self) {
    if !self.modifications {
      self.run = false;
    } else if self
      .wanna_process("You have unsaved modifications. Do you want to exit anyway? (y/n)")
    {
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
