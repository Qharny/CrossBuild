use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Failed to connect to remote build server: {0}")]
    RemoteConnectionError(String),
    
    #[error("iOS build failed: {0}")]
    IosBuildError(String),
    
    #[error("Windows build failed: {0}")]
    WindowsBuildError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Flutter command failed: {0}")]
    FlutterError(String),
    
    #[error("Invalid build configuration: {0}")]
    InvalidConfig(String),
    
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
