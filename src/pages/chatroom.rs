use uuid::Uuid;

use super::*;
const CHAT_AREA_CLASS: &str = "pb-0 flex flex-col overflow-y-auto border-zinc-700 bg-zinc-900";

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const OTHER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-zinc-700 text-white";

#[component]
pub fn ChatRoomPage() -> impl IntoView {
    let (_id, _) = expect_context::<RwSignal<Uuid>>().split();
    let client =
        expect_context::<Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>>>();
    let rx = expect_context::<ReadSignal<String>>();
    let (chat, set_chat) = create_signal(Chat::new());

    create_effect(move |_| {
        let msg = rx.get();
        if !msg.is_empty() {
            set_chat.update(move |c| {
                c.messages.push(Message {
                    user: false,
                    text: msg,
                });
            });
        }
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
                .send(gloo_net::websocket::Message::Text(format!("{}", msg)))
                .await
                .map_err(|_| ServerFnError::ServerError("Websocket issue".to_string()))
        }
    });

    view! {
        <p>"Chatroom"</p>
        <div class="m-5 absolute inset-x-0 bottom-0">
        <ChatArea chat/>
        <TypeArea send/>
        </div>
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
        <div class=CHAT_AREA_CLASS node_ref=chat_div_ref>
        {move || chat.get().messages.iter().map(move |message| {
            let class = if message.user { USER_MESSAGE_CLASS } else { OTHER_MESSAGE_CLASS };
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
                <input class="text-black py-3 my-2 focus:outline-none" type="text" node_ref=input_ref/>
                <button class=BUTTON_CLASS type="submit">"Send"</button>

            </form>
        </div>
    }
}
