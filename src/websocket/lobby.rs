use crate::{api::user::Room, websocket::messages::ChangeRoom};

use super::{messages::Switch, *, ws::WsConn};

use actix::{Actor, Context, Handler, Addr};
use messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use std::collections::{HashMap, HashSet};

pub struct Lobby {
    sessions: HashMap<Uuid, Addr<WsConn>>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Default for Lobby {
    fn default() -> Lobby {
        let mut rooms = HashMap::new();
        rooms.insert(Uuid::nil(), HashSet::new());
        Lobby {
            sessions: HashMap::new(),
            rooms,
        }
    }
}

impl Lobby {
    fn send_message_to_client(&self, message: &str, id_to: &Uuid) {
        log::trace!("Sending {} to {}", message, id_to);
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            log::debug!("attempting to send message but couldn't find user id: {}", id_to);
        }
    }

    fn send_message_to_room(&self, msg: &str, room_id: &Uuid, id: &Uuid) {
        self.rooms
            .get(room_id)
            .unwrap()
            .iter()
            .filter(|conn_id| conn_id.to_owned() != id)
            .for_each(|client_id| self.send_message_to_client(msg, client_id));
    }

    fn remove_client_from_room(&mut self, room_id: &Uuid, id: &Uuid) {
        if let Some(lobby) = self.rooms.get_mut(room_id) {
            if lobby.len() > 1 {
                lobby.remove(id);
            } else if room_id != &HOME_ROOM_ID {
                self.rooms.remove(room_id);
            } else {
                lobby.remove(id);
            }
        }
        log::trace!("Removing {} from {}", id, room_id);
        log::trace!("Sending to {:?}", self.rooms.get(&HOME_ROOM_ID).unwrap());
        self.send_updated_room_info(room_id);
    }

    fn add_client_to_room(&mut self, room_id: Uuid, id: Uuid) {
        self.rooms
            .entry(room_id)
            .or_insert(HashSet::new())
            .insert(id);
        log::trace!("Adding {} to {}", id, room_id);
        self.send_updated_room_info(&room_id);
    }

    fn send_updated_room_info(&self, room_id: &Uuid) {
        let members = self.rooms.get(room_id).unwrap_or(&HashSet::default()).len();
        let msg = Room::json_from_values(room_id.to_owned(), members);
        self.rooms
            .get(&HOME_ROOM_ID)
            .unwrap_or(&HashSet::default())
            .iter()
            .for_each(|client_id| self.send_message_to_client(&msg, client_id));
    }

    fn send_room_info_to_client(&self, client_id: &Uuid) {
        self.rooms.iter().for_each(|entry| {
            let msg = Room::json_from_values(entry.0.to_owned(), entry.1.len());
            self.send_message_to_client(&msg, client_id)
        })
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        if self.sessions.remove(&msg.id).is_some() {
            self.send_message_to_room(
                &format!("{} disconnected.", &msg.username),
                &msg.room_id,
                &msg.id,
            );
            self.remove_client_from_room(&msg.room_id, &msg.id);
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.sessions.insert(msg.id, msg.addr);
        self.send_message_to_client(&format!("{}", msg.id), &msg.id);
        self.add_client_to_room(HOME_ROOM_ID, msg.id);
        self.send_room_info_to_client(&msg.id);
    }
}

impl Handler<Switch> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Switch, _: &mut Self::Context) {
        log::trace!("Switch id: {:?}", msg.id);
        self.remove_client_from_room(&msg.old_room_id, &msg.id);
        self.add_client_to_room(msg.new_room_id, msg.id);
        if let Some(socket_recipient) = self.sessions.get(&msg.id) {
            let _ = socket_recipient.do_send(ChangeRoom(msg.new_room_id));
        }

        if msg.id != HOME_ROOM_ID {
            self.send_message_to_room(
                &format!("{} left the room.", &msg.username),
                &msg.old_room_id,
                &msg.id,
            );
        }
        self.send_message_to_room(
            &format!("{} just joined", &msg.username),
            &msg.new_room_id,
            &msg.id,
        );
        self.send_message_to_client(&format!("your id is {}", msg.id), &msg.id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Self::Context) {
        log::debug!("{:?}", msg.msg);
        if msg.msg.starts_with("\\w") {
            log::debug!("Wisper: {:?}", msg.msg);
            if let Some(id_to) = msg.msg.split(' ').nth(1) {
                self.send_message_to_client(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.send_message_to_room(
                &format!("{}: {}", &msg.username, &msg.msg),
                &msg.room_id,
                &msg.id,
            );
        }
    }
}
