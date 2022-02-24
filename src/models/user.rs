use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};
//use validator::Validate;



#[derive(Debug,Serialize, sqlx::FromRow)]
pub struct User {
    pub id :Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] //to not show the password hash
    pub password_hash : String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
 

}



#[derive(Debug,Serialize, sqlx::FromRow)]
pub struct UserLogin {
    pub id :Uuid,
    pub username: String,
    #[serde(skip_serializing)] //to not show the password hash
    pub password_hash : String,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
 

}
//validate the input fields for new users and update users 
// looking for new ideas
    //  - strongre password
    //  - newusers



#[derive(Debug,Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username : String,
    #[validate(email)]
    pub email : String,
    #[validate(length(min = 7))]
    pub password : String,
}

#[derive(Debug,Deserialize, Validate)]
pub struct UpdateProfile {
    pub full_name:Option<String>,
    pub bio:Option<String>,
    pub image:Option<String>,
}

#[derive(Debug,Deserialize, sqlx::FromRow)]
pub struct PasswordSalt {
    pub salt: String,
}