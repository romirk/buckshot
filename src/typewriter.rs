use std::{io, time};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const FULL_BLOCK: char = '█';
const RECTS: &str = " ▏▎▍▌▋▊▉█";

pub fn typewrite(msg: String) {
    let ten_millis = time::Duration::from_millis(10);
    for c in msg.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        if !c.is_alphanumeric() { continue; }
        sleep(ten_millis);
    }
}

fn len_to_bar(len: u8, cap: u8) -> String {
    let mut s = String::with_capacity(cap as usize);
    if len == 0 {
        s.push_str(&*(0..(cap / 8)).map(|_| ' ').collect::<String>());
        return s;
    }
    s.push_str(&*(0..(len / 8)).map(|_| FULL_BLOCK).collect::<String>());
    if len % 8 != 0 {
        s.push(RECTS.chars().nth((len % 8) as usize).unwrap());
    }
    if cap == 0 || cap <= len {
        return s;
    }
    s.push_str(&*(0..((cap - len) / 8)).map(|_| ' ').collect::<String>());
    s
}

pub fn peek(live: bool) {
    let mut i = 0;
    let col = if live { "\x1b[31;100m" } else { "\x1b[34;100m" };
    print!("{col}");
    while i < 97 {
        let s = len_to_bar(i, 96);
        print!("\r{s}");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(3));
        i += 1;
    }
    i -= 1;
    sleep(Duration::from_millis(300));
    typewrite(format!("\x1b[0m Interesting...{col}"));
    sleep(Duration::from_millis(1000));
    while i > 0 {
        let s = len_to_bar(i, 96);
        print!("\r{s}\x1b[0m Interesting...{col}");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(3));
        i -= 1;
    }
    print!("\x1b[0m\x1b[2K\r");
}
