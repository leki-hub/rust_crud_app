use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;

mod db;
use db::{Post, CreatePost};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&db_url).await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/posts", web::get().to(get_posts))
            .route("/posts/{id}", web::get().to(get_post))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_posts(pool: web::Data<MySqlPool>) -> impl Responder {
    match db::get_posts(&pool).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_post(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match db::get_post_by_id(&pool, id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn create_post(pool: web::Data<MySqlPool>, post: web::Json<CreatePost>) -> impl Responder {
    match db::create_post(&pool, post.into_inner()).await {
        Ok(new_post) => HttpResponse::Created().json(new_post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_post(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>,
    post: web::Json<CreatePost>,
) -> impl Responder {
    match db::update_post(&pool, id.into_inner(), post.into_inner()).await {
        Ok(updated_post) => HttpResponse::Ok().json(updated_post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_post(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match db::delete_post(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
