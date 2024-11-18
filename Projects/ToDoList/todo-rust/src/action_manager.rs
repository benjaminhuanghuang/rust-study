use crate::{display::DisplayManager, task::Task, task_manager::TaskManager};
use std::collections::HashMap;

// automatically implement Debug trait, which allows using the {:?} format specifier
#[derive(Debug)]
pub struct ActionArgs {
  pub command: String,
  pub first: Option<String>,
  pub second: Option<String>,
  pub third: Option<String>,
}

type ActionHandler = fn(am: &mut ActionManager, args: ActionArgs, display: &dyn DisplayManager);

pub struct ActionManager {
  actions: HashMap<&'static str, ActionHandler>,
  manager: TaskManager,
}

impl ActionManager {
  pub fn new() -> ActionManager { 
    ActionManager {
      actions: Self::actions_map(),
      manager: Self::load(),
    }
  }

  pub fn process(&mut self, args: ActionArgs, display: &dyn DisplayManager) -> bool {
    let command = args.command.clone().unwrap();
    match self.select_action(command.as_str()) {
      Ok(f) => f(self, args, display),
      Err(_) => false,
    }
  }

  fn select_action(&mut self, action: &str) -> Result<ActionHandler, &'static str> {
    match self.actions.get(&action) {
      Some(f) => Ok(*f),
      None => Err("Command not found"),
    }
  }

  fn actions_map() -> HashMap<&'static str, ActionHandler> {
    let mut actions = HashMap::new();

    actions.insert("add", ActionManager::add);
    actions.insert("display", ActionManager::dis p  la y);
    actions.insert("remove", ActionManager::remove);
    actions.insert("update", ActionManager::update);
    actions.insert("complete", ActionManager::complete);
    actions.insert("save", ActionManager::save);

    actions
  }
  fn load() -> TaskManager {
    let mut task_manager = TaskManager::new();
    match std::fs::OpenOptions::new().write(false).create(true).read(true).open("tasks.json") {
      Ok(file) => {
        let tasks: Vec<Task> = match serde_json::from_reader(file){
          Ok(t) => t,
          Err(e) => panic!("Error reading tasks: {}", e),
        };
        task_manager.set_tasks(tasks);
        task_manager
    
      }
      Err(_) => TaskManager::new(),
    }
  }

  fn add(&mut self, args: ActionArgs, display: &dyn DisplayManager) -> bool {
    let name = args.first.unwrap();
    let description = args.second.unwrap();

    self.manager.add(name.as_str(), description.as_str());
  }

  fn display(&mut self, _args: ActionArgs, display: &dyn DisplayManager) -> bool {
    for i in 0..self.manager.get_amount() {
      let task = self.manager.get_by(i).unwrap();
      let manager = format!(
        "{}. [{}] - {} - {}\n",
        task.id,
        if task.done { "X" } else { " " },
        task.name,
        task.description
      );
      display.show(manager);
    }
    true
  }

  fn remove(&mut self, args: ActionArgs, _display: &dyn DisplayManager) -> bool {
    let id = args.first.unwrap().parse::<u32>().unwrap();
    self.manager.remove_by(id);
  }

  fn update(&mut self, args: ActionArgs, _display: &dyn DisplayManager) -> bool {
    let id = args.first.unwrap().parse::<u32>().unwrap();
    let name = args.second.unwrap();
    let description = args.third.unwrap();
    self
      .manager
      .update_by(id, name.as_str(), description.as_str());
  }

  fn complete(&mut self, args: ActionArgs, _display: &dyn DisplayManager) -> bool {
    let id = args.first.unwrap().parse::<u32>().unwrap();
    self.manager.complete_by(id);
  }

  fn save(&mut self, _args: ActionArgs, _display: &dyn DisplayManager) -> bool {
    let file = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open("tasks.json")
      .unwrap();

    match file {
      Ok(mut f) => {
        serde_json::to_writer_pretty(f, &self.manager.get_tasks()).unwrap();
        true
      }
      Err(_) => false,
    }
  }
}
