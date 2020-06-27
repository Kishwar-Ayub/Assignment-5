
// use std::fs;
// use futures::{Future, Stream, StreamExt};
use actix_web:: {HttpResponse, Responder,HttpServer,App,web,Result};
use actix_web:: {get,post};
use serde::Deserialize;
// use std::ops::Add::{float};
// use std::io::Write;
//



#[get("api/parameter")]
    async fn take_parameter() -> impl Responder {
        HttpResponse::Ok().body("Taking parameter")
    }

#[get("api/addit/{id}")]
async fn add_parameter(x: web::Path<(f64,)>) -> impl Responder {
    let v = x.0 + 22.0;
    HttpResponse::Ok().body(format!("{} + 22 : {}", x.0, v))
}



#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(l: web::Json<Login>) -> Result<String> {
    // let content = fs::read_to_string().unwrap();

    // let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);

    // l.write(response.as_bytes()).unwrap();
    // l.flush().unwrap();

    if l.username == "kishwar" && l.password == "kishwar" {
    Ok(format!("User successfully signed in as {}", l.username))
    }
    else {
        Ok(format!("User {} is not Authorized",l.username))
    }
}

    
#[actix_rt::main]
async fn main () -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new()
        .service(take_parameter)
        .service(add_parameter)
        .service(web::resource("/postreq.html").route(
         web::get().to(login))
        )
   })
   .bind("127.0.0.1:2020")?
   .run()
   .await
   
 
}


// #[get("/video")]
// async fn get_all_video() -> impl Responder{
//     HttpResponse::Ok().body("ALL VIDEOS!")
// }

// #[get("/video/{id}")]
// async fn get_video(v: web::Path<(u32,)>) -> impl Responder{
//     HttpResponse::Ok().body(format!("Video ID: {}", v.0))
// }
