fn main() {
    if let Err(e) = grater::idle() {
        eprintln!("grater has crashed with the following error: {}", e);
    }
}
