use std::{cell::RefCell, rc::Rc};

use crate::pages::chatroom::ChatRoomPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::notfound::NotFound;
use crate::pages::signup::SignupPage;

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

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Chatroom"/>

        // content for this welcome page
        <Router>
            <main class="bg-zinc-900 text-white h-screen w-screen">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/home" view=HomePage/>
                    <Route path="/chatroom" view=ChatRoomPage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/signup" view=SignupPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// #[component]
// fn LoginSignupForm<T: Clone + ServerFn>(
//     action: Action<T, Result<(), ServerFnError>>,
//     is_login: bool,
// ) -> impl IntoView {
//     view! {
//             <ActionForm action>
//                 <label><b>{"Enter Username:"}</b>
//                     <input type="text" name="username" placeholder="Username" autocomplete="username"/>
//                 </label>
//
//                 <label><b>{"Enter Password:"}</b>
//                     <input type="text" name="password" placeholder="Password" autocomplete={if is_login {"current-password"} else {"new-password"}}/>
//                 </label>
//
//     //            <Show when=move || { !is_login }>
//     //                <label><b>{"Confirm Password:"}</b>
//     //                    <input type="text" name="confirm_password" placeholder="Password" autocomplete="new-password"/>
//     //                </label>
//     //            </Show>
//
//                 <button type="submit">{if is_login {"Login"} else {"Sign Up"}}</button>
//             </ActionForm>
//         }
// }
