/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

use crate::pastebin::{pastebin_create_table, pastebin_get, pastebin_set};

mod pastebin;

#[get("/{id}")]
async fn get_paste(id: web::Path<i64>) -> impl Responder {
    let pastebin = pastebin_get(id.into_inner()).unwrap();
    HttpResponse::Ok().body(pastebin.content)
}

#[post("/")]
async fn set_paste(req_body: String) -> impl Responder {
    let id = pastebin_set(req_body).unwrap();
    format!("{id}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pastebin_create_table().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(get_paste)
            .service(set_paste)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}