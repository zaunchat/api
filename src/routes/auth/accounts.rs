use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginUserSchema {
    email: String,
    password: String
}

#[post("/login")]
pub fn login(body: LoginUserSchema) {

}

#[post("/register")]
pub fn register() {

}

#[get("/verify/<user_id>/<code>")]
pub fn verify(user_id: i64, code: &str) {

}