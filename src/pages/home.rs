use std::rc::Rc;
use uuid::Uuid;

use crate::api::user::{join_room, Room, Rooms};

use super::*;

const A_DIV_CLASS: &str = "";
// "bg-zinc-800 p-16 fixed left-1/3 w-1/3 h-fit text-white";
const A_CLASS: &str = "bg-zinc-600 block py-3.5 my-2 hover:opacity-80";
#[component]
pub fn HomePage() -> impl IntoView {
    let username = expect_context::<ReadSignal<Option<String>>>();
    if username.get().is_none() {
        log::debug!("Username is none");
        leptos_router::use_navigate()("/login", Default::default());
        return view! {}.into_view()
    }

    let rx = expect_context::<ReadSignal<String>>();
    let tx = expect_context::<WriteSignal<String>>();
    let id = expect_context::<ReadSignal<Option<Uuid>>>();

    let (rooms, set_rooms) = create_signal(Rooms::new());

    if id.get().is_none() {
        let client =
            expect_context::<Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>>>();
        let set_id = expect_context::<WriteSignal<Option<Uuid>>>();

        let client_clone = client.clone();
        create_effect(move |_| {
            let location = web_sys::window().unwrap().location();
            let hostname = location.hostname().expect("Failed to get hostname");
            let username = username.get().unwrap();
            let url = format!("ws://{}:3000/ws/chatroom/{}", hostname, username);

            let connection = WebSocket::open(&format!("{}", url))
                .expect("failed to establish websocket connection");

            let (sender, mut recv) = connection.split();
            spawn_local(async move {
                if let Some(msg) = recv.next().await {
                    match msg {
                        Ok(gloo_net::websocket::Message::Text(msg)) => {
                            log::debug!("Msg: {:?}", msg);
                            match Uuid::parse_str(&msg) {
                                Ok(uuid) => {
                                    log::trace!("Setting id: {}", uuid);
                                    set_id.set(Some(uuid))
                                }
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
    };

    create_effect(move |_| match serde_json::from_str::<Room>(&rx.get()) {
        Ok(room) => {
            set_rooms.update(move |r| {
                if room.members > 0 {
                    r.rooms.insert(room.room_id, room.members);
                } else {
                    r.rooms.remove(&room.room_id);
                }
            });
        }
        Err(err) => log::debug!("Error: {:?}", err),
    });
    let join = create_action(move |new_room_id: &Uuid| {
        let new_room_id = new_room_id.to_owned().to_owned();
        async move {
            let id = id.get_untracked();
            log::trace!("Action id: {:?}", id);
            tx.set("".to_owned());
            join_room(id.unwrap(), HOME_ROOM_ID, new_room_id).await.unwrap_or(());
        }
    });

    view! {
        <p>"Home"</p>
        <p>{format!("Username: {}", username.get().unwrap())}</p>
        <div class=A_DIV_CLASS>
        <p class=A_CLASS>{format!("Home Room: {}", rooms.get().rooms.get(&HOME_ROOM_ID).unwrap_or(&0))}</p>
        {move || rooms.get().rooms.iter().map(move |room| {
                let id = room.0.to_owned();
                let members = room.1.to_owned();
                if id != Uuid::nil() {
                    view! {
                        <A class=A_CLASS href="/chatroom" on:click=move |_| {
                            join.dispatch(id);
                        }>{format!("{}: {}", id, members)}</A>
                    }.into_view()
                } else {
                    view! {
                    }.into_view()
                }
            }).collect::<Vec<_>>()

            }
            <A class=A_CLASS href="/chatroom" on:click=move |_| {
                join.dispatch(Uuid::new_v4());
            }>"Create New Room"</A>
            </div>
    }.into_view()
}
