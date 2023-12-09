use super::*;

use actix_web::{get, Error, HttpResponse};
use actix_web_actors::ws;

use crate::websocket::ws::WsConn;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    group_id: web::Path<Uuid>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let srv = &data.into_inner().chat_server;
    let ws = WsConn::new(group_id.into_inner(), srv.clone());

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
