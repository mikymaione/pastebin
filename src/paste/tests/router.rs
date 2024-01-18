/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
mod tests {
    #[actix_web::test]
    async fn full() {
        _full().await;
    }

    async fn _full() {
        let store = paste::store::PastebinStore::new(true)
            .expect("Can not create Pastebin Store");

        let app = actix_web::App::new()
            .configure(|cfg| paste::router::service(cfg, store));

        let app = actix_web::test::init_service(app).await;

        // first element of autoinc is always 1
        let content = String::from("Ciao mi chiamo Michele Maione");
        let resp = actix_web::test::TestRequest::post()
            .uri("/")
            .set_payload(content)
            .send_request(&app)
            .await;

        assert!(resp.status().is_success(), "Failed to create pastebin");
        let id: i64 = actix_web::test::read_body_json(resp).await;

        // get this paste
        let resp = actix_web::test::TestRequest::get()
            .uri(&format!("/{id}"))
            .send_request(&app)
            .await;

        assert!(resp.status().is_success(), "Failed to get pastebin {id}");

        // delete this paste
        let resp = actix_web::test::TestRequest::delete()
            .uri(&format!("/{id}"))
            .send_request(&app)
            .await;

        assert!(resp.status().is_success(), "Failed to delete pastebin {id}");

        // check this paste is none
        let resp = actix_web::test::TestRequest::get()
            .uri(&format!("/{id}"))
            .send_request(&app)
            .await;

        assert!(resp.status().is_client_error(), "Failed to get pastebin {id}");
    }
}