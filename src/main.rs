use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
// use env_logger;

use colaborative_task_app_backend::{
    controllers::{get_all_users, get_user_by_id, insert_task, insert_user, login}, 
    middleware::jwt_validator
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(3600);
        
        let auth = HttpAuthentication::with_fn(jwt_validator);
        
        App::new()
        .wrap(cors)  
        .service(
            web::scope("/api/auth")
                .service(login)
        )
        
        .service(
            web::scope("/api/v1")
                .wrap(auth)
                .service(
                    web::scope("/users")
                    .service(get_all_users)
                    .service(get_user_by_id)
                    .service(insert_user)
                )
                .service(
                    web::scope("/tasks")
                    .service(insert_task)
            )  
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}