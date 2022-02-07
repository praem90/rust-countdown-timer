use std::time::Duration;
use std::thread::sleep;
use std::env;

const ESC: &str = "\x1b[";
const CLEAR: &str = "2J";
const HIDE_CURSOR: &str = "?25l";
const RESET: &str = "H";

fn total_to_minutes(total: u32) -> (u32, u32) {
    let min = total/60;
    let sec = total - min*60;

    return (min, sec);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "timer" {
        timer(args);
    }

    stop_watch();
}

fn timer(args: Vec<String>) {

    let mut total: u32 = args[2].to_string().parse().unwrap();
    total = total * 60;

    loop {
        let (min, sec) = total_to_minutes(total);
        println!("{}{}", ESC, CLEAR);
        println!("{}{}", ESC, HIDE_CURSOR);
        println!("{}{}", ESC, RESET);

        println!("{} mins and {} secs", min, sec);

        total = total-1;

        if total == 0 {
            break;
        }
        sleep(Duration::from_millis(1000));
    }
}

fn stop_watch() {
    let mut total = 0;

    loop {
        let (min, sec) = total_to_minutes(total);
        println!("{}{}", ESC, CLEAR);
        println!("{}{}", ESC, HIDE_CURSOR);
        println!("{}{}", ESC, RESET);

        println!("{} mins and {} secs", min, sec);

        total = total+1;

        sleep(Duration::from_millis(1000));
    }
}
