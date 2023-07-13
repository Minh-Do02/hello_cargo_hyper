#[macro_use] extern crate rocket;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::json::Json;
use hello_cargo::models::{InsertMsg, Msg, PostMsg};
use hello_cargo::schema::msgs::dsl::msgs;
use hello_cargo::establish_connection;
use rocket::Build;
use rocket::Rocket;
use rocket::http::Status;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
#[openapi(skip)]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[openapi]
#[get("/ping")]
fn ping() -> &'static str{
    "pong"
}
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    rocket::build()
        //.mount("/", routes![index, ping, get_msgs, create_msgs])
        .mount("/", openapi_get_routes![index, ping, get_msgs, create_msgs])
        .mount("/swagger-ui/",
               make_swagger_ui(&SwaggerUIConfig {
                   url: "../openapi.json".to_owned(),
                   ..Default::default()
               }),)
}
#[openapi(tag = "Msgs")]
#[get("/msgs")]
fn get_msgs() -> Result<Json<Msg>, Status>{
    let connection: &mut PgConnection = &mut establish_connection();
    msgs
        .select(Msg::as_select())
        .get_result(connection)
        .map(Json)
        .map_err(
            |_| {
                Status::NotFound
            }
        )

}
#[openapi(tag = "Msgs")]
#[post("/msgs", format = "json", data = "<user_input>")]
fn create_msgs(user_input: Json<PostMsg>) -> String {
    let connection: &mut PgConnection = &mut establish_connection();
    let new_msg = InsertMsg{
        title: user_input.title.to_string(),
        body: user_input.body.to_string(),
    };
    diesel::insert_into(msgs).values(&new_msg)
        .returning(Msg::as_returning())
        .get_result(connection).expect("Error post new msg");

    format!("insert done")
}

#[cfg(test)]
mod test{
    use dotenvy::dotenv;
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_ping() {
        dotenv().ok();

        let rocket = rocket::build().mount("/", routes![super::ping]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::ping)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "pong");
    }

    #[test]
    fn test_create_msgs() {
        dotenv().ok();

        let rocket = rocket::build().mount("/", routes![super::create_msgs]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.post(uri!(super::create_msgs)).body(
            r#"{"title": "hello", "body": "hello minh", "published": true}"#
        ) .header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "insert done");
    }
}

