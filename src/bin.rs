extern crate rusty16;

use std::env;
use std::error::Error;

fn main() {
    // TODO(alexyer): Implement proper cli.
    let filename = match env::var("RUSTY16_ROM") {
        Ok(filename) => filename,
        Err(err) => panic!("{:?}", err),
    };

    let mut rusty = rusty16::Rusty16::new();
    rusty.run_rom(&filename);
}
