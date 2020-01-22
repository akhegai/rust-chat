use crate::model::Msg;
use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub id: Uuid,
  pub addr: Recipient<Msg>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: Uuid,
}

pub struct ChatServer {
  sessions: HashMap<Uuid, Recipient<Msg>>,
}

impl ChatServer {
  pub fn new() -> ChatServer {
    ChatServer {
      sessions: HashMap::new(),
    }
  }

  fn send_msg(&self, msg: Msg) {
    self
      .sessions
      .values()
      .for_each(|val| val.do_send(msg.clone()).unwrap());
  }
}

impl Actor for ChatServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
    self.sessions.insert(msg.id, msg.addr);
  }
}

impl Handler<Disconnect> for ChatServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
    self.sessions.remove(&msg.id);
  }
}
