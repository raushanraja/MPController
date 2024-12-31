# MPController

MPController is a Rust-based web application that allows you to control media players using HTTP requests. It uses the Actix-web framework for handling HTTP requests and the MPRIS (Media Player Remote Interfacing Specification) protocol to interact with media players.

## Features

- List all available media players
- Toggle Play/Pause a specific media player using its identity

## ToDos
- Support for other MPRIS commands (e.g. Next, Previous, Stop)
- Support for Seek 

## Project Structure

```tree
.
├── .env
├── .gitignore
├── apitests/
│   └── players.http
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src/
│   ├── config.rs
│   └── main.rs
```

## Getting Started
### Prerequisites

- Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd mpcontroller
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

### Configuration

Create a `.env` file in the root directory with the following content:
```
PORT=8999
RUST_LOG=debug
```

### Running the Application
Start the server:
```sh
cargo run
```

The server will start on the port specified in the `.env` file (default is 8999).
### API Endpoints
- **List Players**
    ```http
    GET /listplayers
    ```
    Returns a JSON array of available media players.
- **Pause Player**
    ```http
    GET /pause/{identity}
    ```
    Toggles Pause/Play the media player with the specified identity.
### Example Requests
You can use the 
players.http file to test the API endpoints using an https://kulala.mwco.app/ nvim plugin.
```http
@hostname=localhost
@port=8999

GET http://{{hostname}}:{{port}}/listplayers

###

GET http://{{hostname}}:{{port}}/pause/Mozilla%20firefox
Content-Type: application/text
```
This project is licensed under the MIT License. See the [LICENSE](https://opensource.org/licenses/MIT) file for details.