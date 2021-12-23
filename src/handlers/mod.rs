use actix_web::{web, web::ServiceConfig, HttpResponse};
mod signin;
mod user;
use crate::errors::AppError;

use signin::signin;
use user::{create_user, me, update_profile};


type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;


pub fn app_config(config : &mut ServiceConfig) {

    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/signin").route(web::post().to(signin));

    let me = web::resource("/me")
        .route(web::get().to(me))
        .route(web::post().to(update_profile));

    config.service(signup).service(auth).service(me);
}
