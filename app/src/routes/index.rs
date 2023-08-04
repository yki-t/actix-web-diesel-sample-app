use crate::Result;
use actix_web::{get, HttpResponse};

#[get("/alive")]
pub async fn get() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("alive"))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    #[actix_web::test]
    async fn can_get_success_alive() {
        let app = test::init_service(App::new().configure(crate::cfg)).await;
        let req = test::TestRequest::get().uri("/alive").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn can_get_proper_alive() {
        let app = test::init_service(App::new().configure(crate::cfg)).await;
        let req = test::TestRequest::get().uri("/alive").to_request();
        let resp = test::call_and_read_body(&app, req).await;
        assert_eq!(resp, bytes::Bytes::from_static(b"alive"));
    }
}
