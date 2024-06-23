# File-based Communication in Rust

This project demonstrates a client-server architecture using file-based communication in Rust. The server will continuously send and receive data by writing to and reading from a file, specifically `~/server.log`. The client will retrieve the responses from the file and display them on the output screen.

## Prerequisites

Before running the application, make sure you have the following installed:

- Rust programming language
- Any text editor or integrated development environment (IDE) of your choice

## Usage

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Open the `server.rs` file and run the server code.
4. Open the `client.rs` file and run the client code.
5. The client will continuously check for new responses in the `~/server.log` file and display them on the output screen.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
