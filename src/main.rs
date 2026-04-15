use actix_web::{App, HttpServer, HttpResponse};
use serde::Serialize;
use reqwest;
use chrono::Utc;
use actix_cors::Cors;


#[derive(serde::Deserialize)]
struct GenderizeResponse {
    name: String,
    gender: Option<String>,
    probability: Option<f64>,
    count: Option<u32>,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    data: Option<Data>,
    message: Option<String>,
}

#[derive(serde::Serialize)]
struct Data {
    name: String,
    gender: String,
    probability: f64,
    sample_size: u32,
    is_confident: bool,
    processed_at: String,
}



#[actix_web::get("/api/classify")]
async fn classify(
    query: actix_web::web::Query<std::collections::HashMap<String, String>>
) -> impl actix_web::Responder {

    let name = query.get("name");

    match name {
        Some(n) => {
            let url = format!("https://api.genderize.io/?name={}", n);

            let response = reqwest::get(&url).await;

            match response {
                Ok(resp) => {
                    let result = resp.json::<GenderizeResponse>().await;

                    match result {
                        Ok(data) => {

                            let gender = match data.gender {
                                Some(g) => g,
                                None => {
                                    return HttpResponse::UnprocessableEntity().body("No prediction available for the provided name");
                                }
                            };

                            let probability = data.probability.unwrap_or(0.0);
                            let count = data.count.unwrap_or(0);
                            let processed_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            let is_confident = probability >= 0.7 && count >= 100;
                            let response_data = Data {
                                name: data.name,
                                gender,
                                probability,
                                sample_size: count,
                                is_confident,
                                processed_at,
                            };
                            let response = ApiResponse {
                                status: "success".to_string(),
                                data: Some(response_data),
                                message: None,
                            };
                            return HttpResponse::Ok().json(response);

                        },
                        Err(_) => {
                            return HttpResponse::InternalServerError().json(ApiResponse {
                                                                            status: "error".to_string(),
                                                                            data: None,
                                                                            message: Some("Failed to parse API response".to_string()),
                                                                        })
                        }
                    }
                },

                Err(_) => {
                    return HttpResponse::InternalServerError().json(ApiResponse {
                                                                    status: "error".to_string(),
                                                                    data: None,
                                                                    message: Some("Upstream API failure".to_string()),
                                                                })
                }
            }
        },

        None => {
            return HttpResponse::BadRequest().json(ApiResponse {status: "error".to_string(), data: None,
            message: Some("Name is required".to_string()),})
        }
    }
}

// #[actix_web::get("/api/classify")]
// async fn classify(query: actix_web::web::Query<std::collections::HashMap<String, String>>) -> impl actix_web::Responder {
//     let name = query.get("name");

//     match name {
//         Some(n) => {
//             let url = format!("https://api.genderize.io/?name={}",n);
//         return HttpResponse::Ok().body(url);

//             let data = Data {
//                 name: n.to_string(),
//                 gender: "unknown".to_string(),
//                 probability: 0.0,
//                 sample_size: 0,
//                 is_confident: false,
//                 processed_at: "2026-04-13 05:23pm".to_string(),
//             };

//             let response = ApiResponse {
//             status: "success".to_string(),
//             data: Some(data),
//             message: None
//             };
//         HttpResponse::Ok().json(response)
//         },

//         None => {
//             let response = ApiResponse {
//             status: "error".to_string(),
//             data: None,
//             message: Some("Name is requires".to_string()),
//             };
//         HttpResponse::BadRequest().json(response)
//         }

//     }

// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(classify)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
