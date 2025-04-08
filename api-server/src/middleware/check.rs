use actix_web::{
    Error, ResponseError,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::{
    future::{Ready, ready},
    rc::Rc,
};

use crate::middleware::jwt::Claims;

use super::error::ApiError;

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
        let headers = request.headers().clone();

        let token_header = headers.get("token");

        if token_header.is_none() {
            let (req, _) = request.into_parts();
            let res = ApiError::Unauthorized(401, "Token no encontrado".to_string())
                .error_response()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
        }

        let token = match token_header.unwrap().to_str() {
            Ok(tok) => tok,
            Err(_) => {
                let (req, _) = request.into_parts();
                let res = ApiError::Unauthorized(401, "Formato del token inválido".to_string())
                    .error_response()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
            }
        };

        let secret_key = std::env::var("JWT_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());
        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(claim) => {
                let exp_ts = claim.claims.exp as i64;
                let exp = chrono::NaiveDateTime::from_timestamp_opt(exp_ts, 0);

                let timezone: Tz = "America/Lima".parse().unwrap();
                let now = Utc::now().with_timezone(&timezone);

                if let Some(exp) = exp {
                    let exp_aware = Utc.from_utc_datetime(&exp).with_timezone(&timezone);
                    if now >= exp_aware {
                        let (req, _) = request.into_parts();
                        let res = ApiError::Unauthorized(401, "El token ha expirado".to_string())
                            .error_response()
                            .map_into_right_body();
                        return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                    }
                } else {
                    let (req, _) = request.into_parts();
                    let res =
                        ApiError::Unauthorized(401, "Fecha de expiración inválida".to_string())
                            .error_response()
                            .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                }

                let fut = self.service.call(request);
                Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
            }
            Err(_) => {
                let (req, _) = request.into_parts();
                let res = ApiError::Unauthorized(401, "Token inválido".to_string())
                    .error_response()
                    .map_into_right_body();
                Box::pin(async { Ok(ServiceResponse::new(req, res)) })
            }
        }
    }
}
