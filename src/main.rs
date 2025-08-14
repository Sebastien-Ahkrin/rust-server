mod driver;
mod server;

use crate::driver::Driver;
use server::Server;

fn main() {
    let mut server = Server::new(8080);

    server.get("/", "index.html");
    server.get("/about", "about.html");

    server.run();
}
