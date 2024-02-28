use std::io::Read;

use crate::{
    db::{self, write_paste},
    AppState, Paste, PasteCategory,
};
use axum::{
    body::Bytes,
    extract::{Json, Multipart, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasteQuery {
    /// The ID of a paste
    paste: String,
}

/// TODO: write structs that this stuff can actually be deserialized to that's not a direct db translation
///
/// read a paste from the database and return it if one was found
pub async fn get_paste(
    State(state): State<AppState>,
    Query(query): Query<PasteQuery>,
) -> Result<Json<Paste>, StatusCode> {
    let found_paste = db::get_paste(&state, &query.paste).await;
    match found_paste {
        // this variable name could be better
        Ok(paste_results) => {
            if let Some(paste) = paste_results {
                return Ok(Json(paste));
            }
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            error!(
                "An error was encountered fetching paste id: {:?}, {:?}",
                query.paste, e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasteSettings {
    id: String,
    category: String,
    /// How long until the paste expires, in seconds.
    duration: u32,
}

/// create a new paste and add it to the database. This endpoint accepts a POST request containing a multipart
/// form with two parts: a `pasteSettings` key containing a serialized [PasteSettings], and a `contents` key containing
// a blob
pub async fn post_paste(State(state): State<AppState>, mut multipart: Multipart) -> StatusCode {
    // a multipart form is used to send both the json containing metadata about the paste,
    // and the paste itself, as a blob
    // https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
    // TODO: increase the max size of the form body
    let mut settings: Option<PasteSettings> = None;
    let mut contents: Option<Bytes> = None;
    // read each part of the form from the file and extract expected values
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let body = field.bytes().await.unwrap();
        match name.as_str() {
            "pasteSettings" => settings = serde_json::from_slice(&body).ok(),
            "contents" => contents = Some(body),
            n => {
                warn!("Received multipart form with an invalid form name: {:?}", n);
                return StatusCode::BAD_REQUEST;
            }
        }
    }

    if settings.is_none() || contents.is_none() {
        warn!("Received multipart form that was missing either the `pasteSettings` key, or the `content` key");
        // we didn't find the right keys, or the paste settings weren't deserialized properly
        return StatusCode::BAD_REQUEST;
    }
    let settings = settings.unwrap();
    // i don't know what hell this is and i don't care
    let contents: Vec<u8> = contents.unwrap().bytes().map(|b| b.unwrap()).collect();
    let category: PasteCategory = match settings.category.try_into() {
        Ok(c) => c,
        Err(e) => {
            warn!("Encountered an issue reading `.category`: {:?}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    match write_paste(
        &state,
        Paste {
            id: settings.id,
            category,
            contents: contents,
            date: DateTime::<Utc>::from(Utc::now()).to_rfc3339(),
            duration: settings.duration,
        },
    )
    .await
    {
        Ok(_) => {
            return StatusCode::CREATED;
        }
        Err(e) => {
            error!("Error encountered when creating paste: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}
