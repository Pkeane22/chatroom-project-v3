use super::{*, messages::ChangeRoom};
use lobby::Lobby;
use messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{self, Message::Text};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
    username: String,
}

impl WsConn {
    pub fn new(room: Uuid, lobby: Addr<Lobby>, username: String) -> WsConn {
        WsConn {
            room,
            lobby_addr: lobby,
            hb: Instant::now(),
            id: Uuid::new_v4(),
            username,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr,
                id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        log::debug!("Stopping message");
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            username: self.username.clone(),
            room_id: self.room,
        });
        Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("disconnect due to failed heartbeat");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    username: act.username.clone(),
                    room_id: act.room,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                log::debug!("Close message");
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                self.lobby_addr.do_send(ClientActorMessage {
                    id: self.id,
                    username: self.username.clone(),
                    msg: s.to_string(),
                    room_id: self.room,
                })
            }
            Err(error) => {
                log::error!("Unexpected Error: {}", error);
                ctx.close(None);
                ctx.stop();
            }
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<ChangeRoom> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: ChangeRoom, _: &mut Self::Context) {
        self.room = msg.0;
    }
}
