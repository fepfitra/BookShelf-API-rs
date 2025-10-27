#[cfg(test)]
mod add_book_with_complete_data {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{build_create_book_request, get_ready_service, new_book_dummy},
    };

    #[tokio::test]
    async fn status_should_be_201() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "success");
        assert_eq!(body["message"], "Buku berhasil ditambahkan");
        assert!(body["data"].is_object());
    }
}

#[cfg(test)]
mod add_book_without_name {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{build_create_book_request, get_ready_service, new_book_dummy},
    };

    fn new_book_no_name() -> Value {
        let book = new_book_dummy();
        if let Value::Object(mut book_obj) = book {
            book_obj.remove("name");
            return Value::Object(book_obj);
        }
        book
    }

    #[tokio::test]
    async fn status_should_be_400() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_no_name());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(
            body["message"],
            "Gagal menambahkan buku. Mohon isi nama buku"
        );
        assert!(!body.as_object().unwrap().contains_key("data"));
    }
}

#[cfg(test)]
mod add_book_with_page_more_than_page_count {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{build_create_book_request, get_ready_service, new_book_dummy},
    };

    fn new_book_overflow_page() -> Value {
        let book = new_book_dummy();
        if let Value::Object(mut book_obj) = book {
            book_obj["pageCount"] = json!(80);
            book_obj["readPage"] = json!(90);
            return Value::Object(book_obj);
        }
        book
    }

    #[tokio::test]
    async fn status_should_be_400() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_overflow_page());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_overflow_page());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_overflow_page());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_overflow_page());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(
            body["message"],
            "Gagal menambahkan buku. readPage tidak boleh lebih besar dari pageCount"
        );
        assert!(!body.as_object().unwrap().contains_key("data"));
    }
}
