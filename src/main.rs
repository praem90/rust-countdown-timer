use std::time::Duration;
use std::thread::sleep;

fn total_to_minutes(total: u32) -> (u32, u32) {
    let min = total/60;
    let sec = total - min*60;

    return (min, sec);
}

fn main() {
    let mut total = 0*60 + 10;
    let esc = "\x1b[";
    let clear = "2J";
    let hide_cursor = "?25l";
    let reset = "H";


    loop {
        let (min, sec) = total_to_minutes(total);
        println!("{}{}", esc, clear);
        println!("{}{}", esc, hide_cursor);
        println!("{}{}", esc, reset);

        println!("{} mins and {} secs", min, sec);

        total = total-1;

        if (total == 0) {
            break;
        }
        sleep(Duration::from_millis(1000));
    }

}
