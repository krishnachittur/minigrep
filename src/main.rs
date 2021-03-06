extern crate minigrep;

use std::env;
use std::process;
use std::fmt::Display;

use minigrep::Config;

fn main() {
    fn errprint<T: Display, V> (err: T) -> V {
        eprintln!("{}", err);
        process::exit(1)
    };
    let config = Config::new(env::args()).unwrap_or_else(errprint);
    minigrep::run(config).unwrap_or_else(errprint);
}
