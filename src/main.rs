use compression::compression;

mod backup_paths;
mod check_delete;
mod compression;
mod email;

fn main() {
    match compression() {
        Ok(log) => {
            check_delete::check_delete();
            email::send_email(&log);
        }
        Err(e) => email::send_email(&e.to_string())
    }
}
