use actix::Message;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Msg {
  pub id: Uuid,
  pub username: String,
  pub body: String,
  pub ts: NaiveDateTime,
}
