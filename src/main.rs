mod actors;
mod model;
use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actors::chat_server::ChatServer;
use actors::ws::WsChatSession;

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(WsChatSession::new(server.get_ref().clone()), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Chat server is starting...");
    let server = ChatServer::new().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/chat/").to(chat_route))
    })
    .bind("10.16.193.75:8000")?
    .run()
    .await
}
