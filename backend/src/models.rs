use crate::schema::{users, tweets};
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "tweets"]
pub struct Tweet {
    pub id: Option<i32>,
    pub userId: Option<i32>,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "tweets"]
pub struct NewTweet {
    pub userId: i32,
    pub title: String,
    pub text: String,
}
