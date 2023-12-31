use std::io;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source};

fn handle_number_input() -> u16 {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let parsed_input_number: u16 = input_line.trim().parse().expect("Input is not valid");

    return parsed_input_number;
}

fn play_audio(msg: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file_to_play: &str = if msg == "focus" { "assets/vineboom.mp3" } else { "assets/tacobellsound.mp3" };
    let file = BufReader::new(File::open(file_to_play).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}

fn focus_time(minutes: u16) {
    let focus_session_time_in_seconds = minutes * 60;
    let focus_time_message: &str = "Time to focus";
    let session_type: &str = "focus";
    session_loop(focus_session_time_in_seconds, focus_time_message);
    play_audio(session_type);
}

fn short_break_time(minutes: u16) {
    let focus_session_time_in_seconds = minutes * 60;
    let focus_time_message: &str = "Take a break ";
    let session_type: &str = "break";
    session_loop(focus_session_time_in_seconds, focus_time_message);
    play_audio(session_type);
}

fn long_break_time(minutes: u16) {
    let focus_session_time_in_seconds = minutes * 60;
    let focus_time_message: &str = "Take a break ";
    let session_type: &str = "break";
    session_loop(focus_session_time_in_seconds, focus_time_message);
    play_audio(session_type);
}

fn default_pomodoro_loop() {
    let focus_session_minutes: u16 = 25;
    let short_break_session_minutes: u16 = 5;
    let long_break_session_minutes: u16 = 30;
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
    long_break_time(long_break_session_minutes);
}

fn customized_session_loop(
    customized_focus_session_minutes: u16, 
    customized_short_break_session_minutes: u16,
    customized_long_break_session_minutes: u16,
    mut customized_number_of_sections_before_long_break: u16,
) {
    while customized_number_of_sections_before_long_break > 0 {
        focus_time(customized_focus_session_minutes);

        if customized_number_of_sections_before_long_break > 1 {
            short_break_time(customized_short_break_session_minutes);
        }

        else if customized_number_of_sections_before_long_break == 1 {
            long_break_time(customized_long_break_session_minutes);
        }

        customized_number_of_sections_before_long_break -= 1;
    }
}

fn hidden_section_loop(
    mut customized_number_of_sections_before_long_break: u8
) {
    while customized_number_of_sections_before_long_break > 0 {
        focus_time(5);
        customized_number_of_sections_before_long_break -= 1;
    }
}

fn session_loop(session_time_in_seconds: u16, msg: &str) -> () {
    let mut elapsed_time = session_time_in_seconds;
    for _i in 0..session_time_in_seconds {
        let minutes = elapsed_time/60;
        let seconds = elapsed_time % 60;
        print!("\r{} {:02}:{:02}", msg, minutes, seconds);
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

    if mode == "1\n" || mode == "\n" {
        default_pomodoro_loop();
    }

    else if mode == "2\n" {
        println!("Duration of the focus session (in minutes): ");
        let customized_focus_session_minutes = handle_number_input();
        println!("Duration of the short break session (in minutes): ");
        let customized_short_break_session_minutes = handle_number_input();
        println!("Duration of the long break session (in minutes): ");
        let customized_long_break_session_minutes = handle_number_input();
        println!("How many sections before taking the long break: ");
        let customized_number_of_sections_before_long_break = handle_number_input();
        
        customized_session_loop(
            customized_focus_session_minutes, 
            customized_short_break_session_minutes,
            customized_long_break_session_minutes,
            customized_number_of_sections_before_long_break,
        );
    }
    
    else if mode == "3\n" { hidden_section_loop(5); }

    println!();
}
