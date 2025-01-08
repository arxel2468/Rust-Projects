use warp::Filter;
use serde::{Serialize, Deserialize};
use warp::http::StatusCode;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    message: String,
}

#[tokio::main]
async fn main() {
    // Serve static files (HTML, CSS, JS) from the "static" directory
    let static_files = warp::fs::dir("static");

    // Route to return a JSON greeting
    let json_greeting = warp::path!("greet")
        .map(|| warp::reply::json(&Greeting {
            message: String::from("Hello from the server!"),
        }));

    // Route to handle dynamic paths (e.g., /greet/John)
    let dynamic_greeting = warp::path!("greet" / String)
        .map(|name: String| warp::reply::json(&Greeting {
            message: format!("Hello, {}!", name),
        }));

    // Route to receive JSON data in a POST request
    let post_greeting = warp::path!("greet" / "post")
        .and(warp::body::json()) // Parse JSON body
        .map(|greeting: Greeting| warp::reply::json(&Greeting {
            message: format!("Hello, {}!", greeting.message),
        }));

    // Route to handle PUT request
    let put_greeting = warp::path!("greet" / "put")
        .and(warp::body::json()) // Parse JSON body
        .map(|greeting: Greeting| {
            format!("Greeting updated to: {}", greeting.message)
        });

    // Route to handle DELETE request
    let delete_greeting = warp::path!("greet" / "delete" / String)
        .map(|name: String| warp::reply::with_status(
            format!("Greeting for {} deleted!", name),
            StatusCode::OK,
        ));

    // Route for handling unknown paths (404 error)
    let not_found = warp::any().map(|| warp::reply::with_status(
        "404 - Not Found",
        StatusCode::NOT_FOUND,
    ));

    // Combine all routes
    let routes = static_files
        .or(json_greeting)
        .or(dynamic_greeting)
        .or(post_greeting)
        .or(put_greeting)
        .or(delete_greeting)
        .or(not_found);

    // Start the server on localhost:3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
