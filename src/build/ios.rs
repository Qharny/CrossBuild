use crate::error::BuildError;
use crate::remote::BuildServer;
use crate::config;

pub async fn build(config: Option<String>, release: bool) -> Result<(), BuildError> {
    let build_config = match config {
        Some(path) => config::load_build_config(&path)?,
        None => config::default_ios_config(),
    };

    let server_config = config::load_server_config()?;
    let server = BuildServer::connect(&server_config)?;

    println!("Starting iOS build...");
    
    // Validate Flutter project
    if !utils::validate_flutter_project()? {
        return Err(BuildError::InvalidConfig("Not a valid Flutter project".into()));
    }

    // Upload project files
    server.upload_project_files()?;

    // Start remote build
    let build_result = server.start_ios_build(build_config, release).await?;

    if !build_result.success {
        return Err(BuildError::IosBuildError(build_result.error_message));
    }

    // Download artifacts
    server.download_artifacts("build/ios")?;

    println!("iOS build completed successfully!");
    Ok(())
}