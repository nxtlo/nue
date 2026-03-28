use nue_web::app;

fn main() {
    if let Err(e) = app() {
        eprintln!("Error occurred: {e}")
    }
}
