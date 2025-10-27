#[cfg(test)]
mod get_all_books {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_get_books_request, get_ready_service, new_book_dummy,
        },
    };

    #[tokio::test]
    async fn response_code_should_be_200() {
        let mut app = app().await;
        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;
        let request = build_get_books_request();
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
        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_should_have_correct_property_and_value() {
        let mut app = app().await;
        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "success");
        assert!(body["data"].is_object());
    }

    #[tokio::test]
    async fn response_body_data_object_should_have_an_array_books_and_contains_one_items() {
        let mut app = app().await;

        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"]["books"].is_array());
        assert_eq!(body["data"]["books"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn the_books_should_have_contains_only_id_name_and_publisher_property_and_value() {
        let mut app = app().await;

        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let books = body["data"]["books"].as_array().unwrap();
        for book in books {
            assert!(book.is_object());
            assert_eq!(book.as_object().unwrap().len(), 3);
            assert!(book.get("id").is_some());
            assert!(book.get("name").is_some());
            assert!(book.get("publisher").is_some());
        }
    }
}

#[cfg(test)]
mod get_detail_books_with_correct_id {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_get_book_by_id_request, build_get_books_request,
            get_ready_service, new_book_dummy,
        },
    };

    #[tokio::test]
    async fn responce_code_should_be_200() {
        let mut app = app().await;

        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;

        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
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
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
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
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "success");
        assert!(body["data"].is_object());
    }

    #[tokio::test]
    async fn response_body_data_object_should_contain_book_object() {
        let mut app = app().await;

        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"]["book"].is_object());
    }

    #[tokio::test]
    async fn the_book_object_should_have_correct_property_and_value() {
        let mut app = app().await;

        let book_payload = new_book_dummy();
        let request = build_create_book_request(book_payload.clone());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_books_request();
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let book_id = body["data"]["books"][0]["id"].as_str().unwrap();
        let request = build_get_book_by_id_request(book_id);
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"]["book"].is_object());
        assert_eq!(body["data"]["book"]["id"].as_str().unwrap(), book_id);
        assert_eq!(body["data"]["book"]["name"], book_payload["name"]);
        assert_eq!(body["data"]["book"]["year"], book_payload["year"]);
        assert_eq!(body["data"]["book"]["author"], book_payload["author"]);
        assert_eq!(body["data"]["book"]["summary"], book_payload["summary"]);
        assert_eq!(body["data"]["book"]["publisher"], book_payload["publisher"]);
        assert_eq!(body["data"]["book"]["pageCount"], book_payload["pageCount"]);
        assert_eq!(body["data"]["book"]["readPage"], book_payload["readPage"]);
        assert_eq!(body["data"]["book"]["reading"], book_payload["reading"]);
    }
}

#[cfg(test)]
mod get_detail_books_with_invalid_id {
    use axum::http::{StatusCode, header};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::Service;

    use crate::{
        app::app,
        services::book::test::{
            build_create_book_request, build_get_book_by_id_request, get_ready_service,
            new_book_dummy,
        },
    };

    #[tokio::test]
    async fn response_code_should_be_404() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_book_by_id_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn response_header_should_be_application_json() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_book_by_id_request("xxxxxx");
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
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_book_by_id_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn response_body_object_should_contain_correct_property_and_value() {
        let mut app = app().await;
        let request = build_create_book_request(new_book_dummy());
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let request = build_get_book_by_id_request("xxxxxx");
        let ready_service = get_ready_service(&mut app).await;
        let response = ready_service.call(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "fail");
        assert_eq!(body["message"], "Buku tidak ditemukan");
    }
}
