pub mod api;
pub mod app;
pub mod pages;

const HOME_ROOM_ID: Uuid = Uuid::nil();

use cfg_if::cfg_if;
use uuid::Uuid;

cfg_if! {
if #[cfg(feature = "hydrate")] {

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    use app::*;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
}
}
cfg_if! {
if #[cfg(feature = "ssr")] {
pub mod appdata;
pub mod websocket;

}
}
