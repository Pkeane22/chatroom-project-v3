pub mod chatroom;
pub mod home;
pub mod login;
pub mod notfound;
pub mod signup;

use crate::api::{
    self,
    user::{Chat, Message},
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use leptos::{
    html::{Div, Input},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use std::{cell::RefCell, rc::Rc};

const BUTTON_CLASS: &str = "bg-zinc-600 py-3.5 my-2 hover:opacity-80";
const LOGIN_SIGNUP_CONTAINER_CLASS: &str =
    "bg-zinc-800 p-16 fixed left-1/3 w-1/3 h-fit text-white text-left";
const INPUT_CLASS: &str =
    "text-black py-3 my-2 focus:outline-none focus:shadow-[0_0_0_2px] focus:shadow-sky-500";

#[component]
fn ErrorComponent(signal: RwSignal<Option<Result<(), ServerFnError>>>) -> impl IntoView {
    {
        move || match signal.get() {
            Some(Err(ServerFnError::ServerError(error))) => {
                view! {<p style="color:red">{error}</p>}.into_view()
            }
            _ => view! {}.into_view(),
        }
    }
}
