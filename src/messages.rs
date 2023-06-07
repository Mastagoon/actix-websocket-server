use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub enum MessageEvent {
    NewClip,
    LatestClips,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub event: MessageEvent,
    pub msg: String,
    pub room_id: Uuid,
}

impl ClientActorMessage {
    pub fn to_json_string(&mut self) -> String {
        serde_json::json!({
            "event": self.event,
            "msg": self.msg,
            "id": self.id,
            "room_id": self.room_id
        })
        .to_string()
    }

    pub fn new(val: &str) -> Result<ClientActorMessage, serde_json::Error> {
        serde_json::from_str::<ClientActorMessage>(val)
    }
}
