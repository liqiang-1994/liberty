use axum::{extract::Multipart, routing::post, Router};

pub async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

// pub async fn save_file<S, E>(path: &str, stream: S)
