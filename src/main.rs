use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
    });

    println!("Serving on http://localhost:8989...");
    server
        .bind("localhost::8989").expect("Error binding server to address")
        .run().expect("Error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
                r#"
                    <title>GDC Calculator</title>
                    <form action="/gdc" method="post">
                        <input type="text" name="n"/>
                        <input type="text" name="m"/>
                        <button type="submit">Compute GCD</button>
                    </form>
                "#
                )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
