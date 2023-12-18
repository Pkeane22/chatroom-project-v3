use std::rc::Rc;
use uuid::Uuid;

use crate::api::user::{join_room, Room, Rooms};

use super::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (id, set_id) = expect_context::<RwSignal<Uuid>>().split();
    let client = expect_context::<Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>>>();
    let rx = expect_context::<ReadSignal<String>>();
    let tx = expect_context::<WriteSignal<String>>();
    let (chat, set_chat) = create_signal(Chat::new());

    let (rooms, set_rooms) = create_signal(Rooms::new());

    let client_clone = client.clone();
    create_effect(move |_| {
        let location = web_sys::window().unwrap().location();
        let hostname = location.hostname().expect("Failed to get hostname");
        let username = "username";
        let url = format!("ws://{}:3000/ws/chatroom/{}", hostname, username);

        let connection =
            WebSocket::open(&format!("{}", url)).expect("failed to establish websocket connection");

        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            if let Some(msg) = recv.next().await {
                match msg {
                    Ok(gloo_net::websocket::Message::Text(msg)) => {
                        log::debug!("{:?}", msg);
                        match Uuid::parse_str(&msg) {
                            Ok(uuid) => set_id.set(uuid),
                            Err(err) => log::debug!("Error: {:?}", err),
                        }
                    }
                    Err(err) => log::debug!("Error: {:?}", err),
                    _ => log::debug!("Unexpected: {:?}", msg),
                }
            }

            while let Some(msg) = recv.next().await {
                log::debug!("{:?}", msg);
                match msg {
                    Ok(gloo_net::websocket::Message::Text(msg)) => {
                        tx.set(msg);
                    }
                    _ => {
                        log::debug!("break: {:?}", msg);
                        break;
                    }
                }
            }
            log::debug!("end of recv");
        });

        *client_clone.borrow_mut() = Some(sender);
    });

    create_effect(move |_| {
        match serde_json::from_str::<Room>(&rx.get()) {
            Ok(room) => {
                set_rooms.update(move |r| {
                    if room.members > 0 {
                        r.rooms.insert(room.room_id, room.members);
                    } else {
                        r.rooms.remove(&room.room_id);
                    }
                });
            }
            Err(err) => log::debug!("{:?}", err),
        }
    });
    let join = create_action(move |new_room_id: &Uuid| {
        let new_room_id = new_room_id.to_owned().to_owned();
        async move {
            join_room(id.get_untracked(), Uuid::nil(), new_room_id)
                .await
                .unwrap_or(());
        }
    });

    view! {
        <p>"Home"</p>
        {move || rooms.get().rooms.iter().map(move |room| {
            let id = room.0.to_owned();
            let members = room.1.to_owned();
            if id != Uuid::nil() {
                view! {
                    <A class=BUTTON_CLASS href="/chatroom" on:click=move |_| {
                        join.dispatch(id);
                    }>{format!("{}: {}", id, members)}</A>
                }.into_view()
            } else {
                view! {
                    <p>{format!("Home Room: {}", members)}</p>
                }.into_view()
            }
        }).collect::<Vec<_>>()

        }
        <A class=BUTTON_CLASS href="/chatroom" on:click=move |_| {
            join.dispatch(Uuid::new_v4());
        }>"Create New Room"</A>
    }
}
