use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use rustls::{ServerConfig, PrivateKey, Certificate};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

struct AppState {
    db: Surreal<Client>,
}

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body(include_str!("../static/index.html"))
// }

#[get("/api/person/{id}")]
async fn get_person(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let db = &data.db;
    match db.select::<Option<Person>>(("person", &id)).await {
        Ok(person) => match person {
            Some(p) => HttpResponse::Ok().json(p),
            None => HttpResponse::NotFound().body("Person not found"),
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Database error occurred")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get SurrealDB address from environment variable, or use default
    let surreal_addr = env::var("SURREAL_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    println!("Connecting to SurrealDB at: {}", surreal_addr);

    // SurrealDB setup
    let db = match Surreal::new::<Ws>(surreal_addr).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to SurrealDB: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };

    match db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    {
        Ok(_) => println!("Successfully signed in to SurrealDB"),
        Err(e) => {
            eprintln!("Failed to sign in to SurrealDB: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database signin failed"));
        }
    }

    match db.use_ns("test").use_db("test").await {
        Ok(_) => println!("Successfully set namespace and database"),
        Err(e) => {
            eprintln!("Failed to set namespace and database: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to set namespace and database"));
        }
    }

    // Insert a test person
    let person = Person { name: "John Doe".to_string(), age: 30 };
    match db.create::<Option<Person>>(("person", "john")).content(person).await {
        Ok(_) => println!("Successfully inserted test person"),
        Err(e) => {
            eprintln!("Failed to insert test person: {:?}", e);
            // We'll continue even if this fails
        }
    }

    // Load TLS keys
    let cert_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/sisawat.com/fullchain.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/sisawat.com/privkey.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    println!("Starting HTTP server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(get_person)
            // Serve static files from the "static" directory
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind_rustls("0.0.0.0:443", config)?
    .bind("0.0.0.0:80")?  // Also bind to port 80 for HTTP
    .run()
    .await
}