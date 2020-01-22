mod actors;
mod model;
use actors::chat_server::ChatServer;

fn main() {
    println!("Hello, world!");
    let server = ChatServer::new();
}
