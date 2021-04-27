extern crate log;
extern crate rusty16;

use std::env;
use env_logger::Env;

fn main() {
    let log_env = Env::default()
        .filter_or("RUSTY16_LOG_LEVEL", "trace")
        .write_style_or("RUSTY16_LOG_STYLE", "always");

    env_logger::init_from_env(log_env);

    // TODO(alexyer): Implement proper cli.
    let filename = match env::var("RUSTY16_ROM") {
        Ok(filename) => filename,
        Err(err) => panic!("{:?}", err),
    };

    rusty16::Rusty16::new()
        .rom_path(&filename)
        .run();
}
