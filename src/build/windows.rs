use crate::error::BuildError;
use std::process::Command;

pub fn build(config: Option<String>, release: bool) -> Result<(), BuildError> {
    let build_config = match config {
        Some(path) => config::load_build_config(&path)?,
        None => config::default_windows_config(),
    };

    println!("Starting Windows build...");

    // Validate Flutter project
    if !utils::validate_flutter_project()? {
        return Err(BuildError::InvalidConfig("Not a valid Flutter project".into()));
    }

    // Run flutter build windows
    let mut command = Command::new("flutter");
    command.arg("build").arg("windows");
    
    if release {
        command.arg("--release");
    }

    let status = command.status()?;
    
    if !status.success() {
        return Err(BuildError::WindowsBuildError("Flutter build failed".into()));
    }

    println!("Windows build completed successfully!");
    Ok(())
}
