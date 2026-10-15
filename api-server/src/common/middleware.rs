use crate::common::errors::ApiError;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub struct JWT;

impl<S, B> Transform<S, ServiceRequest> for JWT
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct CheckLoginMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
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
        let headers = request.headers();
        let token_header = headers.get("token");
        let token = match token_header {
            Some(tok_val) => match tok_val.to_str() {
                Ok(t) => t,
                Err(_) => {
                    let (req, _) = request.into_parts();
                    let res = ApiError::Unauthorized("Formato del token inválido".to_string())
                        .error_response()
                        .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                }
            },
            None => {
                let (req, _) = request.into_parts();
                let res = ApiError::Unauthorized("Token no encontrado".to_string())
                    .error_response()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
            }
        };

        match crate::common::jwt::decodificar_token(token) {
            Ok(claims) => {
                request.extensions_mut().insert(claims);
                let fut = self.service.call(request);
                Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
            }
            Err(err) => {
                let (req, _) = request.into_parts();
                let res = err.error_response().map_into_right_body();
                Box::pin(async { Ok(ServiceResponse::new(req, res)) })
            }
        }
    }
}
