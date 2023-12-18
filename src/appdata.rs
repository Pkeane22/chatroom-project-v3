use actix::Addr;
use leptos::LeptosOptions;
use crate::websocket::lobby::Lobby;
use sqlx::PgPool;

pub struct AppData {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub lobby_addr: Addr<Lobby>,
}

impl AppData {
    pub fn new(leptos_options: LeptosOptions, pool: PgPool, lobby_addr: Addr<Lobby>) -> Self {
        AppData {
            leptos_options,
            pool,
            lobby_addr,
        }
    }
}
