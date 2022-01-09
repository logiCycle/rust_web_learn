use server::Server;

mod server;
mod router;
mod handler;
fn main() {
    let server = Server::new("0.0.0.0:7879");
    server.run();
}
