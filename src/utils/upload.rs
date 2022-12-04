use std::io::Cursor;
use std::time::Duration;
use qiniu_credential::Credential;
use qiniu_upload_manager::{AutoUploader, AutoUploaderObjectParams, UploadManager, UploadTokenSigner};
use crate::model::config::Config;

pub fn upload(config: &Config) {
    let credential = Credential::new(&config.oss.access_key, &config.oss.secret_key);
    let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
        credential,
        &config.oss.bucket_name,
        Duration::from_secs(3600),
    )).build();
    let uploader:AutoUploader = upload_manager.auto_uploader();
    let params = AutoUploaderObjectParams::builder()
        .object_name("resource/test.pdf").file_name("webprogramming.pdf").build();
    uploader.upload_path("/Users/luxyva/Downloads/rust webprogramming.pdf",params).unwrap();
}
