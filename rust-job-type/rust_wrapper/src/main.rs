mod server;
mod handler;
mod health;

fn main() {
    server::serve().unwrap();
}
