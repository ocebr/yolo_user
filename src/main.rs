#[macro_use]
extern crate validator_derive;


mod config;
mod db;
mod errors;
mod handlers;
mod models;
use handlers::app_config;

extern crate serde;
use crate::config::Config;
//use chrono::{DateTime, Utc};
use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Response {
    result: String
}

#[derive(Debug,Serialize, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
    id : String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub pwd : String,
}
#[derive(Debug)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub pass_word: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Inputlogin {
    pub email: String,
    pub pwd : String,
}


//#########################################__response qui marche__################################

//  async fn test_login(item: web::Form<InputUser>) -> impl Responder {
//     // let demo : LoginUser = LoginUser { 
//     //     username:String::from("flo"),
//     //     password:String::from("admin"),
//     //     id: String::from("0001")
//     // };
//     let new_user = NewUser {
//         first_name: &item.fname,
//         last_name: &item.lname,
//         email: &item.email,
//         pass_word : &item.pwd,  
//         created_at: chrono::Local::now().naive_local(),
//     };

//     println!("{}\n",&item.fname);   //test recup les données du form
//     println!("{}\n",&item.lname);
//     println!("{}\n",&item.email);
//     println!("{}\n",&item.pwd);

//     println!("{:?}", new_user);
//     HttpResponse::Ok()
//     .content_type("text/html")
//     .body("
//     <!DOCTYPE html>
//     <html>
//     <head>
//     <title>connexion ok</title>
//     </head>
//     <body>
//     <script>
//     let a = 0;
//     console.log(a);
//     </script>
//     <h1>vous etes connecté</h1>    
//     </body>
//     </html> ")
// }


//  async fn login(item : web::Form::<Inputlogin>) -> impl Responder {
//      println!("email : {} , pass : {}", &item.email, &item.pwd);
//      if &item.email == "aze@aze" && &item.pwd == "allo" {
//         HttpResponse::Ok()
//         .content_type("text/html")
//         .body("
//         <!DOCTYPE html>
//         <html>
//         <head>
//         <title>connexion ok</title>
//         </head>
//         <body>
//         <script>
//         let a = 0;
//         console.log(a);
//         </script>
//         <h1>vous etes connecté</h1>    
//         </body>
//         </html> ")
//      }
//      else {
//         HttpResponse::Ok()
//         .content_type("text/html")
//         .body("
//         <!DOCTYPE html>
//         <html>
//         <head>
//         <title>connexion ok</title>
//         </head>
//         <body>
//         <script>
//         let a = 0;
//         console.log(a);
//         </script>
//         <h1>NON</h1>    
//         </body>
//         </html> ")
//      }
    
// }



 #[actix_web::main]
 async fn main() -> std::io::Result<()> {


    //config
        let config : Config= Config::from_env()
            .expect("error while server configuration");

    //pool (allow connection to be reuse for futures requests)

    let pool = config.db_pool().await.expect("pool error");

    //init the crypto service 

    let crypto_service = config.crypto_service();


     HttpServer::new( move || {
         App::new()
                .wrap(Logger::default())
                .data(pool.clone())
                .data(crypto_service.clone())
                .configure(app_config)
                // .route("/signup", web::post().to(test_login))
                // .route("/login", web::post().to(login))
                 //.route("/addition2", web::post().to(addition))
                 //  .servic e(
                 //     web::resource("/addition2").route(
                 //         web::post().to(addition2)))
     })         

     .bind(format!("{}:{}",config.host,config.port))?
     .run()
     .await
 }

 //https://codepen.io/eridio/pen/BawLmBb
 //https://codepen.io/onediv/pen/WNOdMWw



// oceane veut parler a lethithia
// un quitte la com --> les messages sont transmis au serv
// si le serv est pas la --> la peer to perr continue la discution
// et si  un utilisateur se deco -> message perdu


//postgresql ok 
//inclure

//https://github.com/nemesiscodex/user-auth