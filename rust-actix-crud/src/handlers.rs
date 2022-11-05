use actix_web::{delete, get, post, put, web, Responder, HttpResponse};

#[get("/tweets")]
async fn tweet_index()->impl Responder {
    HttpResponse::Ok().body("Tweet#Index")
}

#[post("/tweets")]
async fn tweet_create()->impl Responder {
    HttpResponse::Ok().body("Tweet#new")
}

#[get("/tweets/{id}")]
async fn tweet_show(id: web::Path<String>)->impl Responder {
    HttpResponse::Ok().body(
        format!("Tweet#show {id}")
    )
}

#[put("/tweets/{id}")]
async fn tweet_update(id: web::Path<String>)->impl Responder {
    HttpResponse::Ok().body(
        format!("Tweet#edit {id}")
    )
}

#[delete("/tweets/{id}")]
async fn tweet_delete(id: web::Path<String>)->impl Responder {
    HttpResponse::Ok().body(
        format!("Tweet#delete {id}")
    )
}
