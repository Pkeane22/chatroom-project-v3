pub mod user;

use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")]{
    pub mod chatroom;

    use actix_web::{web, HttpRequest};
    use uuid::Uuid;

    use crate::appdata::AppData;
    }
}

#[cfg(feature = "ssr")]
fn get_data() -> Result<web::Data<AppData>, ServerFnError> {
    let req = expect_context::<HttpRequest>();
    log::debug!("{:?}", req);
    match req.app_data::<web::Data<AppData>>() {
        None => {
            log::warn!("AppData not found");
            Err(ServerFnError::ServerError("AppData was not found.".into()))
        }
        Some(app_data) => Ok(app_data.clone()),
    }
}
