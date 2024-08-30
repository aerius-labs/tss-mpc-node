# TSS Network

TSS Network is a robust implementation of Threshold Signature Scheme (TSS) for distributed key management and signing operations.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Components](#components)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Security Considerations](#security-considerations)
- [Contributing](#contributing)
- [License](#license)

## Features

- Distributed key generation and management
- Threshold-based signing operations
- Secure communication between signers
- Fault tolerance and Byzantine fault resistance
- Integration with RabbitMQ for message queuing
- MongoDB storage for persistent data
- RESTful API for easy integration
- Comprehensive error handling and logging
- Metrics and monitoring support

## Architecture

The TSS Network consists of two main components:

1. **Manager Service**: Coordinates the signing process, manages signing rooms, and handles API requests.
2. **Signer Service**: Participates in the distributed signing process and interacts with the Manager Service.

## Components

### Manager Service

The Manager Service is responsible for coordinating the signing process, managing signing rooms, and handling API requests. It is implemented in the following files:

rust:src/manager/service.rs
startLine: 1
endLine: 86


### Signer Service

The Signer Service participates in the distributed signing process and interacts with the Manager Service. It is implemented in the following files:

rust:src/signer/service.rs
startLine: 1
endLine: 883


### Common Components

The project includes several common components used by both the Manager and Signer services:

rust:src/common/types.rs
startLine: 53
endLine: 92


## Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/tss-network.git
   cd tss-network
   ```

2. Install Rust and Cargo (if not already installed):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Install dependencies:
   ```
   cargo build
   ```

4. Set up MongoDB and RabbitMQ (refer to their respective documentation for installation instructions).

## Configuration

1. Create a `config` directory in the project root.

2. Create configuration files for different environments:
   - `config/default.toml`
   - `config/development.toml`
   - `config/production.toml`

3. Set the required configuration parameters in these files. Example:

   ```toml
   [database]
   mongodb_uri = "mongodb://localhost:27017/tss_network"

   [queue]
   rabbitmq_uri = "amqp://guest:guest@localhost:5672"

   [manager]
   port = 8080
   signing_timeout = 300

   [signer]
   port = 8081

   [crypto]
   threshold = 2
   total_parties = 3
   ```

4. Set the `RUN_MODE` environment variable to specify the configuration to use:
   ```
   export RUN_MODE=development
   ```

## Usage

### Starting the Manager Service

To start the Manager Service, run:
```
cargo run --bin manager
```

### Starting the Signer Service

To start the Signer Service, run:
```
cargo run --bin signer
```


### API Endpoints

The Manager Service exposes the following API endpoints:

- `POST /sign`: Initiate a signing request
- `GET /status/<request_id>`: Get the status of a signing request
- `GET /signature/<request_id>`: Retrieve the signature for a completed request
- `GET /health`: Health check endpoint

For detailed API usage, refer to the [API Reference](#api-reference) section.

## API Reference

### Initiate Signing Request

**Endpoint:** `POST /sign`

**Request Body:**

```json
{
"message": "48656c6c6f20576f726c64" // Hex-encoded message to sign
}
```

**Response:**
```json
{
"request_id": "550e8400-e29b-41d4-a716-446655440000",
"status": "Pending"
}
```

### Get Signing Status

**Endpoint:** `GET /status/<request_id>`

**Response:**
```json
{
"request_id": "550e8400-e29b-41d4-a716-446655440000",
"status": "Completed"
}
```


### Get Signature

**Endpoint:** `GET /signature/<request_id>`

**Response:**
```json
{
"request_id": "550e8400-e29b-41d4-a716-446655440000",
"signature": "48656c6c6f20576f726c64" // Hex-encoded signature
}
```

## Security Considerations

1. **Key Management**: Ensure that private key shares are securely stored and never transmitted in plain text.
2. **Network Security**: Use TLS/SSL for all network communications between components.
3. **Access Control**: Implement strong authentication and authorization mechanisms for API access.
4. **Secure Configuration**: Keep all configuration files, especially those containing sensitive information, secure and separate from the codebase.
5. **Monitoring and Alerting**: Implement comprehensive logging and monitoring to detect and respond to any suspicious activities.
6. **Regular Audits**: Conduct regular security audits and penetration testing of the system.
7. **Dependency Management**: Regularly update and patch all dependencies to address any known vulnerabilities.

## Contributing

We welcome contributions to the TSS Network project. Please follow these steps to contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them with clear, descriptive messages
4. Push your changes to your fork
5. Submit a pull request to the main repository

Please ensure that your code adheres to the existing style conventions and includes appropriate tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.