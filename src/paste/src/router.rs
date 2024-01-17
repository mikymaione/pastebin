/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use actix_web::{App, delete, get, HttpServer, post, Responder, Result, web};
use actix_web::error::ErrorConflict;

use crate::store::{pastebin_delete, pastebin_get, pastebin_set};

#[get("/{id}")]
async fn get_paste(id: web::Path<i64>) -> Result<impl Responder> {
    let pastebin = pastebin_get(id.into_inner())
        .map_err(ErrorConflict)?;

    let content = pastebin.content;
    Ok(format!("{content}"))
}

#[post("/")]
async fn set_paste(req_body: String) -> Result<impl Responder> {
    let id = pastebin_set(req_body)
        .map_err(ErrorConflict)?;

    Ok(format!("{id}"))
}

#[delete("/{id}")]
async fn delete_paste(id: web::Path<i64>) -> Result<impl Responder> {
    let deleted = pastebin_delete(id.into_inner())
        .map_err(ErrorConflict)?;

    Ok(format!("{deleted}"))
}

pub async fn run_server() -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_paste)
            .service(set_paste)
            .service(delete_paste)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .map_err(anyhow::Error::from)
}