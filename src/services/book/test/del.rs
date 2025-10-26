#[cfg(test)]
mod delete_book_with_correct_id {
    use axum::http::{StatusCode, header, request, response};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_delete_book_request, get_ready_service, new_book_dummy,
        },
    };

    #[tokio::test]
    async fn status_should_be_200() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let book_id = body["data"]["bookId"].as_str().unwrap();

        let request = build_delete_book_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn response_header_should_be_json() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let book_id = body["data"]["bookId"].as_str().unwrap();

        let request = build_delete_book_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
        assert_eq!(content_type, "application/json; charset=utf-8");
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let book_id = body["data"]["bookId"].as_str().unwrap();

        let request = build_delete_book_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let book_id = body["data"]["bookId"].as_str().unwrap();

        let request = build_delete_book_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "success");
        assert_eq!(body["message"], "Buku berhasil dihapus");
    }

    #[tokio::test]
    async fn when_get_detail_books_the_book_should_not_found() {
        let mut app = app();
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let book_id = body["data"]["bookId"].as_str().unwrap();

        let request = build_delete_book_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let _response = ready_service.call(request).await.unwrap();

        let request = crate::services::book::test::build_get_book_by_id_request(book_id);
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod delete_book_with_incorrect_id {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{build_delete_book_request, get_ready_service},
    };

    #[tokio::test]
    async fn status_should_be_404() {
        let mut app = app();
        let request = build_delete_book_request("incorrect-id");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn response_header_should_be_json() {
        let mut app = app();
        let request = build_delete_book_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
        assert_eq!(content_type, "application/json; charset=utf-8");
    }

    #[tokio::test]
    async fn response_body_should_be_an_object() {
        let mut app = app();
        let request = build_delete_book_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app();
        let request = build_delete_book_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(body["message"], "Buku gagal dihapus. Id tidak ditemukan");
    }
}
