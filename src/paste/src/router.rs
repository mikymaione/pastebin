/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use actix_web::{delete, get, post, Responder, Result, web};
use actix_web::error::{ErrorConflict, ErrorNotFound};
use actix_web::web::ServiceConfig;

use crate::store::PastebinStore;

pub struct AppState {
    pub store: PastebinStore,
}

pub fn service(cfg: &mut ServiceConfig, store: PastebinStore) {
    cfg
        .app_data(
            web::Data::new(
                AppState {
                    store
                }
            )
        )
        .service(get_paste)
        .service(set_paste)
        .service(delete_paste);
}

#[get("/{id}")]
async fn get_paste(data: web::Data<AppState>, id: web::Path<i64>) -> Result<impl Responder> {
    let id = id.into_inner();

    let maybe_pastebin = data.store.get(id)
        .map_err(ErrorConflict)?;

    let content = maybe_pastebin
        .map(|p| p.content)
        .ok_or(format!("No pastebin found with id: {id}"))
        .map_err(ErrorNotFound)?;

    Ok(format!("{content}"))
}

#[post("/")]
async fn set_paste(data: web::Data<AppState>, req_body: String) -> Result<impl Responder> {
    let id = data.store.add(req_body)
        .map_err(ErrorConflict)?;

    Ok(format!("{id}"))
}

#[delete("/{id}")]
async fn delete_paste(data: web::Data<AppState>, id: web::Path<i64>) -> Result<impl Responder> {
    let id = id.into_inner();

    let deleted = data.store.delete(id)
        .map_err(ErrorConflict)?;

    Ok(format!("{deleted}"))
}

