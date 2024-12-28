use std::path::Path;
use crate::error::BuildError;

pub fn validate_flutter_project() -> Result<bool, BuildError> {
    if !Path::new("pubspec.yaml").exists() {
        return Ok(false);
    }
    
    // Read pubspec.yaml and verify it's a Flutter project
    let contents = std::fs::read_to_string("pubspec.yaml")?;
    Ok(contents.contains("flutter:"))
}