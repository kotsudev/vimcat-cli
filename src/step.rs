use color_eyre::{eyre::WrapErr, Result};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref STEP: Mutex<Step> = Mutex::new(Step::new());
}

struct Step {
    id: usize,
    names: Vec<String>,
}

impl Step {
    fn new() -> Self {
        Step {
            id: 0,
            names: Vec::new(),
        }
    }

    fn run_step(&mut self, step_name: String, step_fn: fn() -> Result<()>) -> Result<()> {
        let timeout = Duration::from_millis(1000);
        self.id += 1;
        self.names.push(step_name.clone());

        step_fn().wrap_err(format!("step: {}", step_name))?;

        sleep(timeout);

        Ok(())
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn get_current_name(&self) -> Option<String> {
        self.names.last().cloned()
    }
}

// Wrappers for the functions to access RunStepData
pub fn run_step(step_name: String, step_fn: fn() -> Result<()>) -> Result<()> {
    STEP.lock().unwrap().run_step(step_name, step_fn)?;

    Ok(())
}

pub fn get_id() -> usize {
    STEP.lock().unwrap().get_id()
}

pub fn get_names() -> Vec<String> {
    STEP.lock().unwrap().get_names()
}

pub fn get_current_name() -> Option<String> {
    STEP.lock().unwrap().get_current_name()
}
