use actix::Addr;
use leptos::LeptosOptions;
use crate::lobby::Lobby;
use sqlx::PgPool;

pub struct AppData {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub chat_server: Addr<Lobby>,
}

impl AppData {
    pub fn new(leptos_options: LeptosOptions, pool: PgPool, chat_server: Addr<Lobby>) -> Self {
        AppData {
            leptos_options,
            pool,
            chat_server,
        }
    }
}
