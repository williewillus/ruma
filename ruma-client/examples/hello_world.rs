use std::{convert::TryFrom, env, process::exit};

use http::Uri;
use ruma::{
    api::client::r0::{alias::get_alias, membership::join_room_by_id, message::send_message_event},
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        EventType,
    },
    RoomAliasId,
};
use ruma_client::{self, Client};
use serde_json::value::to_raw_value as to_raw_json_value;

async fn hello_world(homeserver_url: Uri, room_alias: &RoomAliasId) -> anyhow::Result<()> {
    let client = Client::new(homeserver_url, None);

    client.register_guest().await?;
    let response = client.request(get_alias::Request { room_alias }).await?;

    let room_id = response.room_id;

    client.request(join_room_by_id::Request::new(&room_id)).await?;

    client
        .request(send_message_event::Request {
            room_id: &room_id,
            event_type: EventType::RoomMessage,
            txn_id: "1",
            data: to_raw_json_value(&MessageEventContent::Text(TextMessageEventContent {
                body: "Hello World!".to_owned(),
                formatted: None,
                relates_to: None,
            }))?,
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (homeserver_url, room) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            eprintln!("Usage: {} <homeserver_url> <room>", env::args().next().unwrap());
            exit(1)
        }
    };

    hello_world(homeserver_url.parse()?, &RoomAliasId::try_from(room.as_str())?).await
}
