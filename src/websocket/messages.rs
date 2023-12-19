use super::{*, ws::WsConn};
use actix::{Message, Addr};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);


#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsConn>,
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Switch {
    pub id: Uuid,
    pub username: String,
    pub old_room_id: Uuid,
    pub new_room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub username: String,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub username: String,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChangeRoom(pub Uuid);
