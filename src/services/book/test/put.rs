#[cfg(test)]
mod update_book_with_complete_data {
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_get_book_by_id_request, build_update_book_request,
            get_ready_service, new_book_dummy, update_book_dummy,
        },
    };

    #[tokio::test]
    async fn status_code_should_be_200() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "success");
        assert_eq!(body["message"], "Buku berhasil diperbarui");
    }

    #[tokio::test]
    async fn when_get_details_books_book_object_should_contain_updated_values() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let updated_book = update_book_dummy();
        let request = build_update_book_request(id, updated_book.clone());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = build_get_book_by_id_request(id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let book = &body["data"]["book"];

        assert_eq!(book["name"], updated_book["name"]);
        assert_eq!(book["year"], updated_book["year"]);
        assert_eq!(book["author"], updated_book["author"]);
        assert_eq!(book["summary"], updated_book["summary"]);
        assert_eq!(book["publisher"], updated_book["publisher"]);
        assert_eq!(book["pageCount"], updated_book["pageCount"]);
        assert_eq!(book["readPage"], updated_book["readPage"]);
        assert_eq!(book["reading"], updated_book["reading"]);
    }
}

#[cfg(test)]
mod update_book_without_name {
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_update_book_request, get_ready_service,
            new_book_dummy, update_book_dummy,
        },
    };

    fn update_book_no_name() -> serde_json::Value {
        let book = update_book_dummy();
        if let serde_json::Value::Object(mut book_obj) = book {
            book_obj.remove("name");
            return serde_json::Value::Object(book_obj);
        }
        book
    }

    #[tokio::test]
    async fn status_code_should_be_400() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(
            body["message"],
            "Gagal memperbarui buku. Mohon isi nama buku"
        );
        assert!(!body.as_object().unwrap().contains_key("data"));
    }
}

#[cfg(test)]
mod update_book_with_page_read_more_than_page_count {
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_update_book_request, get_ready_service,
            new_book_dummy, update_book_dummy,
        },
    };

    fn update_book_page_read_more_than_page_count() -> serde_json::Value {
        let mut book = update_book_dummy();
        if let serde_json::Value::Object(ref mut book_obj) = book {
            book_obj.insert("readPage".to_string(), serde_json::Value::from(300));
            return serde_json::Value::Object(book_obj.clone());
        }
        book
    }

    #[tokio::test]
    async fn status_code_should_be_400() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_page_read_more_than_page_count());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_page_read_more_than_page_count());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_page_read_more_than_page_count());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = body["data"]["bookId"].as_str().unwrap();

        let request = build_update_book_request(id, update_book_page_read_more_than_page_count());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(
            body["message"],
            "Gagal memperbarui buku. readPage tidak boleh lebih besar dari pageCount"
        );
        assert!(!body.as_object().unwrap().contains_key("data"));
    }
}

#[cfg(test)]
mod update_book_with_invalid_id {
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_update_book_request, get_ready_service,
            new_book_dummy, update_book_dummy,
        },
    };

    #[tokio::test]
    async fn status_code_should_be_404() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_update_book_request("xxxxxx", update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_update_book_request("xxxxxx", update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_update_book_request("xxxxxx", update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_update_book_request("xxxxxx", update_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(
            body["message"],
            "Gagal memperbarui buku. Id tidak ditemukan"
        );
        assert!(!body.as_object().unwrap().contains_key("data"));
    }
}
