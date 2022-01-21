use std::{env, process::exit};
use douyu::{Conf, context};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    // let env = Env::new().filter("MY_LOG").write_style("MY_LOG_STYLE");

    // env_logger::init_from_env(env);

    run(Conf { host: "openapi-danmu.douyu.com", port: 8601});
    
}
fn run(conf: Conf) {

    let room = context::Room::new(276200, conf);
    // let room = context::Room::new(4489985, conf);
    if let Err(e) = room.login() {
        log::error!("{}", e);
        exit(1);
    }
    room.ping();
    room.revice_msg();
}
