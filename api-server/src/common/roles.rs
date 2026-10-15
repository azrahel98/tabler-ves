use crate::common::errors::ApiError;
use crate::common::jwt::Claims;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub struct RequiereEditor;

impl<S, B> Transform<S, ServiceRequest> for RequiereEditor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequiereEditorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequiereEditorMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequiereEditorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequiereEditorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let method = request.method();
        // Para métodos distintos de GET, HEAD y OPTIONS, se exige rol ADMIN o EDITOR
        if method != Method::GET && method != Method::HEAD && method != Method::OPTIONS {
            let role = request
                .extensions()
                .get::<Claims>()
                .map(|c| c.role.to_uppercase());

            match role.as_deref() {
                Some("ADMIN") | Some("EDITOR") => {}
                _ => {
                    let (req, _) = request.into_parts();
                    let res = ApiError::Forbidden("No tiene permisos para modificar información".to_string())
                        .error_response()
                        .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                }
            }
        }

        let fut = self.service.call(request);
        Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::jwt::generate_token;
    use crate::common::middleware::JWT;
    use actix_web::{test, web, App, HttpResponse, Responder};

    async fn dummy_handler() -> impl Responder {
        HttpResponse::Ok().body("ok")
    }

    #[actix_web::test]
    async fn test_roles_middleware() {
        unsafe { std::env::set_var("JWT_KEY", "secret_key_de_prueba_para_test_1234567890") };

        let app = test::init_service(
            App::new().service(
                web::scope("/test")
                    .wrap(RequiereEditor)
                    .wrap(JWT)
                    .route("/item", web::get().to(dummy_handler))
                    .route("/item", web::post().to(dummy_handler))
                    .route("/item", web::delete().to(dummy_handler)),
            ),
        )
        .await;

        // 1. Sin token → 401
        let req = test::TestRequest::get().uri("/test/item").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        // Generar tokens para cada rol
        let token_user = generate_token(1, "USER".into(), "user@test.com".into(), "Usuario".into(), None)
            .expect("Error al generar token user");
        let token_editor = generate_token(2, "EDITOR".into(), "editor@test.com".into(), "Editor".into(), None)
            .expect("Error al generar token editor");
        let token_admin = generate_token(3, "ADMIN".into(), "admin@test.com".into(), "Admin".into(), None)
            .expect("Error al generar token admin");

        // 2. USER GET → 200
        let req = test::TestRequest::get()
            .uri("/test/item")
            .insert_header(("token", token_user.as_str()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // 3. USER POST → 403
        let req = test::TestRequest::post()
            .uri("/test/item")
            .insert_header(("token", token_user.as_str()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403);

        // 4. EDITOR POST → 200
        let req = test::TestRequest::post()
            .uri("/test/item")
            .insert_header(("token", token_editor.as_str()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // 5. ADMIN DELETE → 200
        let req = test::TestRequest::delete()
            .uri("/test/item")
            .insert_header(("token", token_admin.as_str()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
