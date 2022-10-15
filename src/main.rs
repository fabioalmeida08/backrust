mod backup_paths;
mod check_and_delete;
mod compression;
mod email;
mod move_and_sync;
mod s3;

use crate::check_and_delete::check_delete;
use crate::compression::compression;
use crate::email::send_email;
use crate::move_and_sync::move_and_sync;
use crate::s3::{remove_old_objects_from_s3, upload_object_s3};
use aws_sdk_s3::Client;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let bucket = env::var("BUCKET").unwrap();

    match compression().await {
        Ok(f) => {
            let log = upload_object_s3(&f, &client, &bucket).await;
            _ = check_delete();
            _ = move_and_sync(&f);
            _ = remove_old_objects_from_s3(&client, &bucket).await;
            _ = send_email(&log.unwrap())
        }
        Err(e) => {
            print!("erro => {}", e)
        }
    }
}
