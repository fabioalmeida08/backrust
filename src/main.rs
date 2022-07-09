mod compression;
mod email;
fn main() {
    let log = compression::compression().unwrap();
    email::send_email(&log);
}
