use std::collections::HashMap;


use super::*;

cfg_if! {
if #[cfg(feature = "ssr")]{
    use actix_web::http::StatusCode;
    use leptos_actix::ResponseOptions;
    use sqlx::FromRow;
    use crate::websocket::messages::Switch;

    const HASH_COST: u32 = 5;
}
}

#[cfg(feature = "ssr")]
#[derive(Serialize, FromRow)]
struct User {
    id: Uuid,
    username: String,
    hash_password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    last_login_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "ssr")]
impl User {
    fn new(username: String, hash_password: String) -> User {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            hash_password,
            created_at: now,
            last_login_at: now,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Serialize, FromRow)]
struct HashPassword {
    hash_password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chat {
    pub messages: Vec<Message>,
}
impl Chat {
    pub fn new() -> Chat {
        Chat {
            messages: Vec::<Message>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rooms {
    pub rooms: HashMap::<Uuid, usize>,
}
impl Rooms {
    pub fn new() -> Rooms { 
        Rooms { 
            rooms: HashMap::<Uuid, usize>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub room_id: Uuid,
    pub members: usize,
}

impl Room {
    pub fn json_from_values(room_id: Uuid, members: usize) -> String {
        let room = Room { 
            room_id,
            members,
        };
        serde_json::to_string(&room).unwrap()
    }
}

#[server[LoginUser, "/api"]]
pub async fn login_user(username: String, password: String) -> Result<(), ServerFnError> {
    let data = get_data()?;

    let result = sqlx::query_as::<_, HashPassword>(
        r#"
        SELECT hash_password FROM users
        WHERE username = $1
        "#,
    )
    .bind(&username)
    .fetch_one(&data.pool)
    .await;

    match result {
        Err(error) => {
            log::info!("Error on first query: {:?}.", error);
            Err(ServerFnError::ServerError(
                "Internal Server Error.".to_string(),
            ))
        }
        Ok(hash_password) => match bcrypt::verify(&password, &hash_password.hash_password) {
            Err(error) => {
                log::info!("Error on hash: {:?}.", error);
                Err(ServerFnError::ServerError(
                    "Internal Server Error.".to_string(),
                ))
            }
            Ok(false) => {
                log::info!("Incorrect password entered for {}", &username);
                Err(ServerFnError::ServerError(
                    "Username or password is incorrect.".to_string(),
                ))
            }
            Ok(true) => {
                let now = chrono::Utc::now();
                let result = sqlx::query(
                    r#"
                    UPDATE users
                    SET last_login_at = $1
                    WHERE username = $2
                    "#,
                )
                .bind(&now)
                .bind(&username)
                .execute(&data.pool)
                .await;

                match result {
                    Err(error) => {
                        log::warn!("Error on second query: {:?}.", error);
                        Err(ServerFnError::ServerError(
                            "Internal Server Error.".to_string(),
                        ))
                    }
                    Ok(_) => {
                        log::info!("User successfully logged in.");
                        leptos_actix::redirect("/home");
                        Ok(())
                    }
                }
            }
        },
    }
}

#[server[SignupUser, "/api"]]
pub async fn signup_user(
    username: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    if password != confirm_password {
        return Err(ServerFnError::ServerError(
            "Passwords do not match.".to_string(),
        ));
    };

    let response = expect_context::<ResponseOptions>();
    let data = get_data()?;

    let hash_password = bcrypt::hash(&password, HASH_COST).unwrap();
    let user = User::new(username, hash_password);

    let result = sqlx::query(
        r#"
        INSERT INTO users (id, username, hash_password, created_at, last_login_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.hash_password)
    .bind(&user.created_at)
    .bind(&user.last_login_at)
    .execute(&data.pool)
    .await;

    match result {
        Err(sqlx::error::Error::Database(error)) => match error.kind() {
            sqlx::error::ErrorKind::UniqueViolation => {
                log::info!("Error: Username already exists.");
                Err(ServerFnError::ServerError(
                    "Username already exists.".to_string(),
                ))
            }
            _ => {
                log::warn!(
                    "Unexpected error kind:\n\tError: {:?}\n\tKind: {:?}",
                    error,
                    error.kind()
                );
                Err(ServerFnError::ServerError(
                    "Internal Server Error.".to_string(),
                ))
            }
        },
        Err(error) => {
            log::warn!("Error: {:?}", error);
            Err(ServerFnError::ServerError(
                "Internal Server Error.".to_string(),
            ))
        }
        Ok(_) => {
            log::info!("User {} successfully created.", &user.username);
            response.set_status(StatusCode::CREATED);
            leptos_actix::redirect("/home");
            Ok(())
        }
    }
}

#[server(JoinRoom, "/api")]
pub async fn join_room(id: Uuid, old_room_id: Uuid, new_room_id: Uuid) -> Result<(), ServerFnError> {
    let data = get_data()?;

    data.lobby_addr.do_send(Switch {
        id,
        username: "username".to_owned(),
        old_room_id,
        new_room_id,
    });
    Ok(())
}
