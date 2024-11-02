use std::io::{stdout, BufWriter, StdoutLock};
use ferris_says::say;

fn main() {
    let stdout = stdout();
    let mut message = String::from("Hello, world");
    let mut width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    ferris_print(&mut message, width, &mut writer);

    message = String::from("Hi, this is a new message!");
    width = message.chars().count();
    ferris_print(&mut message, width, &mut writer);
}

fn ferris_print(message: &mut String, width: usize, mut writer: &mut BufWriter<StdoutLock>) {
    let _ = match say(&message, width, &mut writer) {
        Ok(_) => println!("All good, input: {}, {}", message, width),
        Err(error) => println!("An error occurred: {}", error),
    };
}
