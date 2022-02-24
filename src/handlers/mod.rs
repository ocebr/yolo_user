use actix_web::{web, web::ServiceConfig, HttpResponse};
mod signin;
use crate::errors::AppError;

use signin::signin;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;


pub fn app_config(config : &mut ServiceConfig) {

    let signin = web::resource("/signin").route(web::post().to(signin));

    config.service(signin);
}