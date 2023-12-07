use crate::appdata::AppData;
use actix_web::{
    get,
    web::{Data, Path, Payload},
    Error,
};
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::ws::WsConn;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    group_id: Path<Uuid>,
    data: Data<AppData>,
) -> Result<HttpResponse, Error> {
    let srv = &data.into_inner().chat_server;
    let ws = WsConn::new(group_id.into_inner(), srv.clone());

    match ws::start(ws, &req, stream) {
        Err(resp) => {
            log::debug!("Error: {:?}", resp);
            Err(resp)
        },
        Ok(resp) => {
            log::debug!("Ok: {:?}", resp);
            Ok(resp)
        },
    }
}
