mod server;

use server::Server;

fn main() {
    Server::new(8080).run();
}
