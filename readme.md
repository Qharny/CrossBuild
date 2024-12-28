# Flutter Cross-Builder

A powerful CLI tool that enables developers to build Flutter applications for iOS and Windows platforms from a Windows environment. This tool bridges the gap for users who need access to macOS for iOS builds and simplifies the Windows build process.

## Features

- **Cross-Platform Building**: Build iOS applications without requiring a local macOS machine
- **Windows Build Support**: Streamlined Windows build process with enhanced error handling
- **Remote Build Server Integration**: Secure communication with remote build servers for iOS builds
- **Custom Build Configurations**: Support for custom build settings and configurations
- **Detailed Error Reporting**: Comprehensive error handling and reporting system

## Prerequisites

- Rust 1.70 or higher
- Flutter SDK installed and in PATH
- Windows 10/11 operating system
- Internet connection for iOS builds
- Access to a compatible remote build server for iOS builds

## Installation

1. Clone the repository:
```bash
git clone https://github.com/qharny/CrossBuild
cd CrossBuild
```

2. Build the project:
```bash
cargo build --release
```

3. (Optional) Add to PATH for global access:
```bash
# Add the binary location to your system's PATH
```

## Usage

### Configuration

Before using the iOS build feature, configure your remote build server:

```bash
CrossBuild configure --server "https://your-build-server.com" --api-key "your-api-key"
```

### Building for iOS

```bash
# Debug build
CrossBuild ios

# Release build
CrossBuild ios --release

# Custom configuration
CrossBuild ios --config custom_config.json --release
```

### Building for Windows

```bash
# Debug build
CrossBuild windows

# Release build
CrossBuild windows --release

# Custom configuration
CrossBuild windows --config custom_config.json --release
```

## Configuration Files

### Server Configuration

The server configuration is stored in `%APPDATA%/CrossBuild/config.json`:

```json
{
  "server_url": "https://your-build-server.com",
  "api_key": "your-api-key"
}
```

### Build Configuration

Custom build configurations can be specified in JSON format:

```json
{
  "target_platform": "ios",
  "build_mode": "release",
  "extra_flags": [
    "--no-codesign",
    "--verbose"
  ]
}
```

## Error Handling

The tool provides detailed error messages for common issues:

- Remote server connection failures
- Build process errors
- Configuration issues
- Flutter command execution problems
- Project validation errors

## Development

### Project Structure

```
CrossBuild/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── error.rs
│   ├── build/
│   │   ├── mod.rs
│   │   ├── ios.rs
│   │   └── windows.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── templates/
│   │       ├── ios_default.json
│   │       └── windows_default.json
│   ├── remote/
│   │   └── mod.rs
│   └── utils/
│       └── mod.rs
├── tests/
│   ├── build_tests.rs
│   ├── config_tests.rs
│   └── remote_tests.rs
└── README.md
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test build_tests

# Run with logging
RUST_LOG=debug cargo test
```

## Troubleshooting

### Common Issues

1. **Remote Server Connection Failed**
   - Verify your internet connection
   - Check server URL and API key configuration
   - Ensure the remote server is operational

2. **Flutter Command Not Found**
   - Verify Flutter is installed
   - Check if Flutter is in your PATH
   - Run `flutter doctor` to verify your setup

3. **Build Configuration Errors**
   - Validate JSON syntax in configuration files
   - Ensure all required fields are present
   - Check file permissions

### Logging

For detailed logging, set the RUST_LOG environment variable:

```bash
RUST_LOG=debug CrossBuild ios --release
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Flutter team for the amazing framework
- Rust community for the robust tooling
- All contributors who helped shape this project

## Contact

For support or queries:
- Create an issue in the repository
- Email: kabuteymanasseh5@gmail.com
- Twitter: @mr_kabuteyy