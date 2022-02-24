use crate::{config::crypto::CryptoService,
    models::user::{User,NewUser,UpdateProfile,UserLogin},
    errors::AppError,
    };


use actix_web::{web::Data, FromRequest};
use sqlx::{PgPool, postgres::PgQueryAs};
use std::sync::Arc;
use std::ops::Deref;
use color_eyre::Result;
use uuid::Uuid;
use futures::future::{ready,Ready};
use tracing::instrument;

pub struct UserRepository {
    pool: Arc<PgPool>
}

impl UserRepository {
    pub fn new(pool:Arc<PgPool>) -> Self {
        Self {pool}
    }

#[instrument(skip(self))]
    pub async fn find_by_username(&self, username: &str) -> Result<UserLogin> {
        let maybe_user = sqlx::query_as::<_, UserLogin>("select * from users_pass where username = $1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(maybe_user)
    }

}

impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(    
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
