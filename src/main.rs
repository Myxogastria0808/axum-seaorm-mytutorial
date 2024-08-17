use ::entity::{post, post::Entity as Post};
use axum::{routing::get, Router};
use sea_orm::*;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5433/test_db";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = api().await;
}

//axum
async fn api() {
    //Router((
    let app = Router::new().route("/create", get(create_post_handler));
    //Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//Handler
async fn create_post_handler() -> String {
    create_post().await.expect("Creat post is failed");
    "Creat post is success".to_string()
}

async fn create_post() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    //insert
    let post_model = post::ActiveModel {
        content: Set("Hello World from SeaORM".to_owned()),
        ..Default::default()
    };
    let post = Post::insert(post_model).exec(&db).await?;
    println!("Inserted post: {:?}", post);
    Ok(())
}
