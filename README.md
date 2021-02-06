# Rust HTTP Server

## What does it do?
This is a basic HTTP Server running in a multi-threaded style. There's a Listener thread that receives TCP connections at port 3000, then uses message-passing to send TCPStream objects to several Worker threads. 

For now, the workers read from the incoming buffer and print it to the console, along with which worker processed it.

## How do I test it?

Open two terminals. 

### Terminal 1
Run `cargo run .` to run the service. After 5 seconds it will terminate.

### Terminal 2
While the server is running, use `curl` or your favourite http request sender to reach out to localhost:3000.