use std::cell::RefCell;
use std::rc::Rc;

use crate::api;
use crate::pages::chatroom::ChatRoomPage;
use crate::pages::home::HomePage;
use crate::pages::loginsignup::LoginSignupPage;
use crate::pages::notfound::NotFound;

use futures::stream::SplitSink;
use gloo_net::websocket::futures::WebSocket;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> =
        Default::default();
    let (id, set_id) = create_signal::<Option<Uuid>>(None);
    let (username, set_username) = create_signal::<Option<String>>(None);
    let (rx, tx) = create_signal("".to_owned());

    provide_context(client.clone());
    provide_context(id);
    provide_context(username);
    provide_context(rx);

    provide_context(set_id);
    provide_context(set_username);
    provide_context(tx);

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
                    <Route path="/login" view=|| {
                        let action = create_server_action::<api::user::LoginUser>();
                        view! {
                            <LoginSignupPage
                                is_signup=false
                                action=action
                            />
                        }
                    }/>
                    <Route path="/signup" view=|| {
                        let action = create_server_action::<api::user::SignupUser>();
                        view! {
                            <LoginSignupPage
                                is_signup=true
                                action=action
                            />
                        }
                    }/>
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
