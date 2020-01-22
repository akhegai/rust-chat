use crate::actors::chat_server::{ChatServer, Connect, Disconnect};
use crate::model::Msg;
use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub struct WsChatSession {
  id: Uuid,
  ts: Instant,
  chat_server: Addr<ChatServer>,
}

impl Actor for WsChatSession {
  type Context = WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.heartbeat(ctx);

    self.id = Uuid::new_v4();

    self.chat_server.do_send(Connect {
      id: self.id,
      addr: ctx.address().recipient(),
    })
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    self.chat_server.do_send(Disconnect { id: self.id });
    Running::Stop
  }
}

impl Handler<Msg> for WsChatSession {
  type Result = ();

  fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) {
    ctx.text(serde_json::to_string(&msg).unwrap());
  }
}
impl StreamHandler<Result<Message, ProtocolError>> for WsChatSession {
  fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
    let msg = match msg {
      Err(_) => {
        ctx.stop();
        return;
      }
      Ok(msg) => msg,
    };

    println!("WEBSOCKET MESSAGE: {:?}", msg);
    match msg {
      Message::Ping(msg) => {
        self.ts = Instant::now();
        ctx.pong(&msg);
      }
      Message::Pong(_) => {
        self.ts = Instant::now();
      }
      Message::Binary(_) => println!("Unexpected binary"),
      Message::Close(_) => {
        ctx.stop();
      }
      Message::Continuation(_) => {
        ctx.stop();
      }
      Message::Nop => (),

      Message::Text(text) => {
        let msg: Msg = serde_json::from_str(&text).unwrap();
        self.chat_server.do_send(msg);
      }
    }
  }
}

impl WsChatSession {
  pub fn new(chat_server: Addr<ChatServer>) -> WsChatSession {
    WsChatSession {
      id: Uuid::new_v4(),
      ts: Instant::now(),
      chat_server,
    }
  }

  fn heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
    ctx.run_interval(Duration::from_secs(5), |act, ctx| {
      if Instant::now().duration_since(act.ts) > Duration::from_secs(10) {
        println!("Websocket Client heartbeat failed, disconnecting!");

        act.chat_server.do_send(Disconnect { id: act.id });

        ctx.stop();

        return;
      };

      ctx.ping(b"");
    });
  }
}
