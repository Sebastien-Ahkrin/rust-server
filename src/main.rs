mod driver;
mod server;

use server::Server;

fn main() {
    let mut server = Server::new(8080);

    server.get("/", "index.html");
    server.get("/about", "about.html");
    server.fallback("404.html");

    server.run();
}
