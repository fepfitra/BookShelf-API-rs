#[cfg(test)]
mod add_book_with_complete_data {
    use axum::{
        Router,
        body::Body,
        http::{Method, Request, StatusCode, header},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::{Service, util::ServiceExt};

    use crate::app::app;

    fn default_book_payload() -> Value {
        json!({
            "name": "Buku Baru",
            "year": 2025,
            "author": "Penulis Hebat",
            "summary": "Ringkasan buku...",
            "publisher": "Penerbit A",
            "pageCount": 100,
            "readPage": 10,
            "reading": false
        })
    }

    async fn get_ready_service(app: &mut Router) -> &mut Router {
        ServiceExt::<Request<Body>>::ready(app)
            .await
            .expect("Service should be ready")
    }

    fn build_create_book_request(payload: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri("/books")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn status_should_be_201() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
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
    use axum::{
        Router,
        body::Body,
        http::{Method, Request, StatusCode, header},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::{Service, util::ServiceExt};

    use crate::app::app;

    fn default_book_payload() -> Value {
        json!({
            "year": 2025,
            "author": "Penulis Hebat",
            "summary": "Ringkasan buku...",
            "publisher": "Penerbit A",
            "pageCount": 100,
            "readPage": 10,
            "reading": false
        })
    }

    async fn get_ready_service(app: &mut Router) -> &mut Router {
        ServiceExt::<Request<Body>>::ready(app)
            .await
            .expect("Service should be ready")
    }

    fn build_create_book_request(payload: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri("/books")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn status_should_be_400() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
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
    use axum::{
        Router,
        body::Body,
        http::{Method, Request, StatusCode, header},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::{Service, util::ServiceExt};

    use crate::app::app;

    fn default_book_payload() -> Value {
        json!({
            "name": "Buku Baru",
            "year": 2025,
            "author": "Penulis Hebat",
            "summary": "Ringkasan buku...",
            "publisher": "Penerbit A",
            "pageCount": 100,
            "readPage": 110, // readPage lebih besar dari pageCount
            "reading": false
        })
    }

    async fn get_ready_service(app: &mut Router) -> &mut Router {
        ServiceExt::<Request<Body>>::ready(app)
            .await
            .expect("Service should be ready")
    }

    fn build_create_book_request(payload: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri("/books")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn status_should_be_400() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(default_book_payload());
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
