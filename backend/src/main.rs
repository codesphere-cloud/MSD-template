#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{get, post, put, web, App, HttpServer, Responder, HttpResponse};
use crate::models::{User, Tweet, NewTweet, Comment, NewComment};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::{users::dsl as users_dsl, tweets::dsl as tweets_dsl, comments::dsl as comments_dsl};
use std::env;

mod schema;
mod models;
use crate::schema::users::dsl::*;


#[get("/backend")]
async fn hello() -> impl Responder {
    "Hello, world!!!!"
}

#[post("/backend/users")]
async fn create_user(new_user: web::Json<User>) -> impl Responder {
    let mut conn = establish_connection();

    // Fügen Sie den neuen Benutzer ein
    diesel::insert_into(users)
        .values(new_user.into_inner())
        .execute(&mut conn)
        .expect("Error inserting new user");

    // Manuell die ID des eingefügten Benutzers abrufen
    let inserted_user = users.order(id.desc()).first::<User>(&mut conn)
        .expect("Failed to fetch inserted user");

    HttpResponse::Created().json(inserted_user)
}

// Handler zum Abrufen aller Benutzer
#[get("/backend/users")]
async fn get_users() -> impl Responder {
    let mut conn = establish_connection();
    let users_list = users
        .load::<User>(&mut conn)
        .expect("Error loading users");

    HttpResponse::Ok().json(users_list)
}

// Handler zum Abrufen eines einzelnen Benutzers anhand seiner ID
#[get("/backend/users/{id}")]
async fn get_user_by_id(user_id: web::Path<i32>) -> impl Responder {
    let user_id = user_id.into_inner();
    
    let mut conn = establish_connection();

    let user = users
        .filter(schema::users::id.eq(user_id))
        .first::<User>(&mut conn)
        .expect("Error loading user");

    HttpResponse::Ok().json(user)
}

// Handler zum Aktualisieren eines Benutzers
#[put("/backend/users/{id}")]
async fn update_user(_user_id: web::Path<i32>, updated_user: web::Json<User>) -> impl Responder {
    let mut conn = establish_connection();
    let target = users.filter(schema::users::id.eq(id.nullable()));
    diesel::update(target)
        .set(updated_user.into_inner())
        .execute(&mut conn)
        .expect("Error updating user");

    HttpResponse::Ok().body("User updated successfully")
}

// Handler zum Erstellen eines neuen Tweets
#[post("/backend/tweets")]
async fn create_tweet(new_tweet: web::Json<NewTweet>) -> impl Responder {
    let mut conn = establish_connection();
    diesel::insert_into(tweets_dsl::tweets)
        .values(new_tweet.into_inner())
        .execute(&mut conn)
        .expect("Error inserting new tweet");

    HttpResponse::Created().body("Tweet created successfully")
}

#[get("/backend/tweets")]
async fn get_tweets() -> impl Responder {

    let connection = establish_connection();

    // Query für den Join von `tweets` und `users`
    let tweets_with_users = tweets
        .inner_join(users)
        .select((
            tweets::id,
            tweets::userId,
            tweets::title,
            tweets::likes,
            tweets::dislikes,
            tweets::text,
            users::name,  // Feld für den Benutzernamen aus der `users` Tabelle
        ))
        .load::<TweetWithUser>(&connection)
        .expect("Error loading tweets");

    HttpResponse::Ok().json(tweets_with_users)
}

// Handler zum Erstellen eines neuen Kommentars
#[post("/backend/comments")]
async fn create_comment(new_comment: web::Json<NewComment>) -> impl Responder {
    let mut conn = establish_connection();
    diesel::insert_into(comments_dsl::comments)
        .values(new_comment.into_inner())
        .execute(&mut conn)
        .expect("Error inserting new comment");

    HttpResponse::Created().body("Comment created successfully")
}

// Handler zum Abrufen aller Kommentare für einen bestimmten Tweet
#[get("/backend/tweets/{tweet_id}/comments")]
async fn get_comments_for_tweet(tweet_id: web::Path<i32>) -> impl Responder {
    let tweet_id = tweet_id.into_inner();
    let mut conn = establish_connection();
    
    let comments_list = comments_dsl::comments
        .filter(comments_dsl::tweetId.eq(tweet_id))
        .load::<Comment>(&mut conn)
        .expect("Error loading comments");

    HttpResponse::Ok().json(comments_list)
}


// Datenbankverbindung initialisieren
fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(establish_connection()))
            .service(create_user)
            .service(get_user_by_id)
            .service(get_users)
            .service(update_user)
            .service(create_tweet)
            .service(get_tweets)
            .service(hello)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
