mod compression;
mod email;
mod backup_paths;
mod check_delete;

fn main() {
    let log = compression::compression().unwrap();
    check_delete::check_delete();
    email::send_email(&log);
}
