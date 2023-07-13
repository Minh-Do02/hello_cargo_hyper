use diesel::prelude::*;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;



#[derive(Queryable, Selectable, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Msg {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::msgs)]

pub struct InsertMsg{
    pub title : String,
    pub body: String,
}
#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]

pub struct PostMsg{
    pub title: String,
    pub body: String,
    pub published: bool,

}



