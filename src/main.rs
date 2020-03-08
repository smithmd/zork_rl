use std::process;

fn main() {
    if let Err(e) = zork_rl::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
