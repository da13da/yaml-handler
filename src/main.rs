mod config;
mod actions;

use std::error::Error;
use std::fs;
use config::Config;
use actions::run_actions;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml_content = fs::read_to_string("example.yaml")?;
    let config: Config = serde_yaml::from_str(&yaml_content)?;
    println!("{:#?}", config);

    run_actions(config.steps)?;

    Ok(())
}
