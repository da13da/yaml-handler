mod move_action;
mod update_json;
mod command;

use crate::config::Step;
use std::error::Error;

pub fn run_actions(steps: Vec<Step>) -> Result<(), Box<dyn Error>> {
    for step in steps {
        match step {
            Step::Move {src, dest, .. } => {
                move_action::execute(&src, &dest)?;
            }
            Step::UpdateJson { file, field, value, .. } => {
                update_json::execute(&file, &field, &value)?;
            }
            Step::Command { command, .. } => {
                command::execute(&command)?;
            }
        }
    }
    Ok(())
}
