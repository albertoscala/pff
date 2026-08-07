mod puller;

use crate::puller::Puller;

fn main() {
    // Retrive puller file
    let puller = Puller::fetch();

    // Pull the entrier
    puller.pull();
}
