use crate::schema::{users, tweets, comments};
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "tweets"]
pub struct Tweet {
    pub id: Option<i32>,
    pub userId: Option<i32>,
    pub title: String,
    pub likes: Option<i32>,
    pub dislikes: Option<i32>,
    pub text: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "tweets"]
pub struct NewTweet {
    pub userId: i32,
    pub title: String,
    pub text: String,
}

// Neue Strukturen für Kommentare hinzufügen
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "comments"]
pub struct Comment {
    pub id: Option<i32>,
    pub userId: Option<i32>,
    pub tweetId: i32,
    pub likes: Option<i32>,
    pub dislikes: Option<i32>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "comments"]
pub struct NewComment {
    pub userId: Option<i32>,
    pub tweetId: i32,
}

