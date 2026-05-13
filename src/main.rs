use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::{
    sync::Mutex,
    thread,
    time::Duration,
};

// Data suhu boiler
struct AppState {
    temperature: Mutex<f32>,
    status: Mutex<String>,
}

// Dashboard HTML
async fn dashboard(data: web::Data<AppState>) -> impl Responder {
    let temp = *data.temperature.lock().unwrap();
    let status = data.status.lock().unwrap().clone();

    let color = if status == "NORMAL" {
        "green"
    } else if status == "WARNING" {
        "orange"
    } else if status == "OVERHEAT" {
        "red"
    } else {
        "blue"
    };

    let html = format!(
        r#"
        <html>
        <head>
            <title>Monitoring Boiler</title>
            <meta http-equiv="refresh" content="2">
            <style>
                body {{
                    font-family: Arial;
                    background-color: #f4f4f4;
                    text-align: center;
                    padding-top: 50px;
                }}

                .card {{
                    width: 500px;
                    margin: auto;
                    background: white;
                    padding: 30px;
                    border-radius: 15px;
                    box-shadow: 0px 0px 15px rgba(0,0,0,0.2);
                }}

                h1 {{
                    color: #333;
                }}

                .temp {{
                    font-size: 60px;
                    margin: 20px;
                    color: #222;
                }}

                .status {{
                    font-size: 35px;
                    font-weight: bold;
                    color: {};
                }}
            </style>
        </head>

        <body>
            <div class="card">
                <h1>SISTEM MONITORING SUHU BOILER</h1>

                <div class="temp">{:.1} °C</div>

                <div class="status">{}</div>
            </div>
        </body>
        </html>
        "#,
        color, temp, status
    );

    HttpResponse::Ok()
    .content_type("text/html; charset=UTF-8") //Tentukan charset di sini
    .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        temperature: Mutex::new(160.0),
        status: Mutex::new(String::from("NORMAL")),
    });

    // Clone data untuk thread monitoring
    let monitoring_data = app_state.clone();

    // Thread monitoring suhu
    thread::spawn(move || {

        let mut rng = rand::thread_rng();
        let mut naik = true;

        loop {
            {
                let mut temp = monitoring_data.temperature.lock().unwrap();
                let mut status = monitoring_data.status.lock().unwrap();

                // Simulasi suhu naik/turun
                if naik {
                    *temp += rng.gen_range(5.0..15.0);
                } else {
                    *temp -= rng.gen_range(5.0..15.0);
                }

                // Overheat
                if *temp > 200.0 {
                    *status = String::from("OVERHEAT");
                    println!("ALARM OVERHEAT AKTIF!");
                }

                // Warning
                else if *temp > 180.0 {
                    *status = String::from("WARNING");
                }

                // Suhu rendah
                else if *temp < 150.0 {
                    *status = String::from("SUHU TERLALU RENDAH");
                    println!("ALARM SUHU RENDAH AKTIF!");
                }

                // Normal
                else {
                    *status = String::from("NORMAL");
                }

                // Pendingin otomatis
                if *temp >= 220.0 {
                    println!("SISTEM PENDINGIN DIAKTIFKAN");
                    *temp = 170.0;
                    naik = false;
                }

                // Pemanas otomatis
                if *temp <= 120.0 {
                    println!("SISTEM PEMANAS DIAKTIFKAN");
                    *temp = 160.0;
                    naik = true;
                }

                println!("Suhu Boiler: {:.1} °C | Status: {}", *temp, *status);
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("Server berjalan di http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(dashboard))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}