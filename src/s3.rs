use crate::check_and_delete::compare_dates;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error, PKG_VERSION};
use regex::Regex;

use std::path::Path;
use std::process;

async fn upload_object(
    client: &Client,
    bucket: &str,
    filename: &str,
    key: &str,
) -> Result<(), Error> {
    let body = ByteStream::from_path(Path::new(filename)).await;

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;

            println!("Upload success. Version: {:?}", resp.version_id.unwrap());
        },
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}

pub async fn upload_object_s3(filename: &str, client: &Client, bucket: &str) -> Result<(), Error> {
    println!();

    let key = &filename;

    if true {
        println!("S3 client version: {}", PKG_VERSION);
        println!("Bucket:            {}", &bucket);
        println!("Filename:          {}", &filename);
        println!("Key:               {}", &key);
        println!();
    }

    upload_object(&client, &bucket, &filename, &key).await
}

pub async fn remove_old_objects_from_s3(client: &Client, bucket: &str) -> Result<(), Error> {
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let mut backups: Vec<String> = vec![];

    let re = Regex::new(r"\w*.tar.gz\b").unwrap();

    for object in resp.contents().unwrap() {
        if re.is_match(&object.key().unwrap().to_string()) {
            backups.push(object.key().unwrap().to_string());
        }
    }

    for object in backups {
        let object_date = &object.replace(".tar.gz", "");
        let days = compare_dates(&object_date);
        if days > 30 {
            client
                .delete_object()
                .bucket(bucket)
                .key(&object)
                .send()
                .await?;

            println!("Object {} deleted.", &object);
        }
    }

    Ok(())
}