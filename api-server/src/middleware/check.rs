use actix_web::{
    Error, HttpMessage, ResponseError,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::{
    future::{Ready, ready},
    rc::Rc,
};

use crate::{key::key::JWT_KEY, middleware::jwt::Claims};

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
            let res = ApiError::Unauthorized("Token no encontrado".to_string())
                .error_response()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
        }

        let token = match token_header.unwrap().to_str() {
            Ok(tok) => tok,
            Err(_) => {
                let (req, _) = request.into_parts();
                let res = ApiError::Unauthorized("Formato del token inválido".to_string())
                    .error_response()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
            }
        };

        let secret_key = JWT_KEY;
        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(claim) => {
                let claims = claim.claims;
                let exp_ts = claims.exp as i64;
                let exp = chrono::DateTime::from_timestamp(exp_ts, 0).map(|dt| dt.naive_utc());

                let timezone: Tz = "America/Lima".parse().unwrap();
                let now = Utc::now().with_timezone(&timezone);

                if let Some(exp) = exp {
                    let exp_aware = Utc.from_utc_datetime(&exp).with_timezone(&timezone);
                    if now >= exp_aware {
                        let (req, _) = request.into_parts();
                        let res = ApiError::Unauthorized("El token ha expirado".to_string())
                            .error_response()
                            .map_into_right_body();
                        return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                    }
                } else {
                    let (req, _) = request.into_parts();
                    let res = ApiError::Unauthorized("Fecha de expiración inválida".to_string())
                        .error_response()
                        .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
                }

                request.extensions_mut().insert(claims);

                let fut = self.service.call(request);
                Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
            }
            Err(_) => {
                let (req, _) = request.into_parts();
                let res = ApiError::Unauthorized("Token inválido".to_string())
                    .error_response()
                    .map_into_right_body();
                Box::pin(async { Ok(ServiceResponse::new(req, res)) })
            }
        }
    }
}
