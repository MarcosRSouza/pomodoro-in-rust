use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn focus_time() {
    let focus_session_time_in_seconds = 25 * 60;
    let focus_time_message: &str = "Time to focus";
    session_loop(focus_session_time_in_seconds, focus_time_message);
}

fn break_time() {
    let focus_session_time_in_seconds = 5 * 60;
    let focus_time_message: &str = "Take a break ";
    session_loop(focus_session_time_in_seconds, focus_time_message);
}

fn session_loop(session_time_in_seconds: u16, msg: &str) -> () {
    let mut elapsed_time = session_time_in_seconds;
    for _i in 0..session_time_in_seconds {
        let minutes = elapsed_time/60;
        let seconds = elapsed_time % 60;
        print!("\r{} {}:{}", msg, minutes, seconds);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
        elapsed_time -= 1;
    }
}

fn main() {
    println!("Pomodoro App with Rust");

    println!("Choose the configuration mode:");
    println!("[1]: 25/5 sessions (default)");
    println!("[2]: custom");

    let mut mode = String::new();

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    if mode == "1\n" {
        focus_time();
        break_time();
        focus_time();
    }
    println!();
}
