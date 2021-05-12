use actix_web::web::{self, Json};
use actix_web::{get, post, App, Error, HttpResponse, HttpServer};
use diesel::{query_dsl::methods::LimitDsl, RunQueryDsl};
use dotenv::dotenv;
use server::create_db_pool;
use server::models::{NewPost, Post};
use server::DbPool;

#[post("/posts")]
async fn create_post(pool: web::Data<DbPool>, form: Json<NewPost>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("failed to access db from pool");

    let post = web::block(move || Post::new(&conn, &form.title, &form.text))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(post))
}

#[get("/posts")]
async fn get_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("failed to access db from pool");

    let posts = web::block(move || {
        use server::schema::posts::dsl::*;
        posts.limit(50).load::<Post>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(posts))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = create_db_pool();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| {
            match v.parse::<u16>() {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("PORT ({}) is invalid: {}", v, e);
                    None
                }
            }
        })
        .unwrap_or(3000);

    println!("View @ localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_post)
            .service(get_posts)
    })
    .bind(&format!("127.0.0.1:{}", port))?
    .run()
    .await
}
