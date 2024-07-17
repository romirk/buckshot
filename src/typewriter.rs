use std::{io, time};
use std::io::Write;
use std::thread::sleep;

pub fn typewrite(msg: String) {
    let ten_millis = time::Duration::from_millis(10);
    for c in msg.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        if !c.is_alphanumeric() { continue }
        sleep(ten_millis);
    }
}