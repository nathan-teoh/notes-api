use axum::Router;
use axum::Json;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api",get(instructions))
        .route("/api/notes",post(new_note))
        .route("/api/notes/:id",get(get_note_by_id).delete(delete_note_by_id));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn instructions()->Json<String>{
    todo!()
}

async fn new_note()->Json<String>{
    todo!()
}

async fn get_note_by_id()->Json<String>{
    todo!()
}

async fn delete_note_by_id()->Json<String>{
    todo!()
}