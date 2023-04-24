use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};

fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

fn get_users() -> impl Responder {
    HttpResponse::Ok().body("[Alice, Bob]")
}

fn put_users() -> impl Responder {
    // here do some logic to put a new user
    HttpResponse::Ok().body("success")
}

fn say_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let resp = format!("hello {}", name);
    HttpResponse::Ok().body(resp)
}

fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

struct AppData {
    tmpl: Tera
}


fn main() {
    HttpServer::new(|| {
        let tera =
            Tera::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
            ).unwrap();

        App::new()
            .app_data(Data::new(AppData {tmpl: tera}))
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .service(
                web::resource("/users")
                    .route(web::get().to(get_users))
                    .route(web::put().to(put_users))
            )
            .service(
                web::resource("/hello/{name}")
                    .route(web::get().to(say_hello))
            )
            .service(
                web::resource("/tmpl/{name}")
                    .route(web::get().to(render_tmpl))
            )
    })
    .bind("127.0.0.1:7000")
    .unwrap()
    .run()
    .unwrap();
}