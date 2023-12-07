use std::{cell::RefCell, rc::Rc};

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
            <main>
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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {<p>"Home"</p>}
}

#[component]
fn ChatRoomPage() -> impl IntoView {
    let (chat, set_chat) = create_signal(Chat::new());

    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> =
        Default::default();

    let client_clone = client.clone();
    create_effect(move |_| {
        let location = web_sys::window().unwrap().location();
        let hostname = location.hostname().expect("Failed to get hostname");
        let id = "12345678901234567890123456789012";
        let url = format!("ws://{}:3000/ws/chatroom/{}", hostname, id);

        let connection =
            WebSocket::open(&format!("{}", url)).expect("failed to establish websocket connection");

        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(gloo_net::websocket::Message::Text(msg)) => {
                        set_chat.update(move |c| {
                            c.messages.push(Message {
                                user: false,
                                text: msg,
                            });
                        });
                    }
                    _ => {
                        break;
                    }
                }
            }
        });

        *client_clone.borrow_mut() = Some(sender);
    });

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_chat.update(move |c| {
            c.messages.push(user_message);
        });

        let client_clone = client.clone();
        let msg = new_message.to_string();
        async move {
            client_clone
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(gloo_net::websocket::Message::Text(format!(
                    "{}: {}",
                    "Username", msg
                )))
                .await
                .map_err(|_| ServerFnError::ServerError("Websocket issue".to_string()))
        }
    });

    view! {
        <p>"Chatroom"</p>
        <ChatArea chat/>
        <TypeArea send/>
    }
}

#[component]
fn ChatArea(chat: ReadSignal<Chat>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        chat.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div node_ref=chat_div_ref>
        {move || chat.get().messages.iter().map(move |message| {
            let class = if message.user { "bg-blue-500 text-white" } else { "bg-zinc-700 text-white" };
            view! {
                <div class=class>{message.text.clone()}</div>
            }
        }).collect::<Vec<_>>()

        }
        </div>
    }
}

#[component]
fn TypeArea(send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    view! {
        <div>
            <form on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input doesn't exist");
                send.dispatch(input.value());
                input.set_value("");

            }>
                <input type="text" node_ref=input_ref/>
                <button type="submit">"Send"</button>

            </form>
        </div>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<api::user::LoginUser>();

    view! {
        <div class="container">
            <ActionForm action=login_user>
                <label><b>{"Enter Username:"}</b>
                    <input type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=login_user.value()/>
                <button type="submit">"Login"</button>
            </ActionForm>
            <div class="a_container">
                <p>"Don't have an account?"</p>
                <a href="/signup">"Sign Up"</a>
            </div>
        </div>
    }
}

#[component]
fn SignupPage() -> impl IntoView {
    let signup_user = create_server_action::<api::user::SignupUser>();

    view! {
        <div class="container">
            <ActionForm action=signup_user>
                <label><b>{"Enter Username:"}</b>
                    <input type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <label><b>{"Confirm Password:"}</b>
                    <input type="text"
                        name="confirm_password"
                        placeholder="Confirm Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=signup_user.value()/>
                <button type="submit">"Sign Up"</button>
            </ActionForm>
            <div class="a_container">
                <p>"Already have an account?"</p>
                <a href="/login">"Login"</a>
            </div>
        </div>
    }
}

#[component]
fn LoginSignupForm<T: Clone + ServerFn>(
    action: Action<T, Result<(), ServerFnError>>,
    is_login: bool,
) -> impl IntoView {
    view! {
            <ActionForm action>
                <label><b>{"Enter Username:"}</b>
                    <input type="text" name="username" placeholder="Username" autocomplete="username"/>
                </label>

                <label><b>{"Enter Password:"}</b>
                    <input type="text" name="password" placeholder="Password" autocomplete={if is_login {"current-password"} else {"new-password"}}/>
                </label>

    //            <Show when=move || { !is_login }>
    //                <label><b>{"Confirm Password:"}</b>
    //                    <input type="text" name="confirm_password" placeholder="Password" autocomplete="new-password"/>
    //                </label>
    //            </Show>

                <button type="submit">{if is_login {"Login"} else {"Sign Up"}}</button>
            </ActionForm>
        }
}

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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
