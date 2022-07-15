mod compression;
mod email;
mod backup_paths;
fn main() {
    let log = compression::compression().unwrap();
    email::send_email(&log);
}
