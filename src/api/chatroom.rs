use super::*;

use actix_web::{get, Error, HttpResponse};
use actix_web_actors::ws;

use crate::websocket::ws::WsConn;

#[get("/{username}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    username: web::Path<String>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let lobby_addr = &data.into_inner().lobby_addr;
    let ws = WsConn::new(Uuid::nil(), lobby_addr.clone(), username.into_inner());

    match ws::start(ws, &req, stream) {
        Err(resp) => {
            log::debug!("Error: {:?}", resp);
            Err(resp)
        }
        Ok(resp) => {
            log::debug!("Ok: {:?}", resp);
            Ok(resp)
        }
    }
}
