mod server;
mod driver;

use server::Server;
use crate::driver::Driver;

fn main() {
    Server::new(8080).run();
}
