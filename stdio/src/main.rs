use std::io::{Read, StdinLock};
use std::time::Duration;
use libbrainpipe::{map_brainpipe, MyOptions};
fn main() -> Result<(),String> {
    let full_args = std::env::args().collect::<Vec<_>>();
    let (progname, args) = full_args.split_first().expect("arguments!");
    let matches = libbrainpipe::parse_options(args, &progname, my_println as fn(String)).map_err(|_| "exiting")?;

    let path = matches.free.get(0).expect("a bf file path should be given as the first free argument");
    let file = std::fs::read_to_string(path)
        .map_err(|err| format!("the bf file should be valid and existent and accessible. Rust says: {:?} on reading path {path}",err))?;

    let mut program_chars = file.chars().peekable();
    let stdin = std::io::stdin().lock(); 

    let mut main_iterator = map_brainpipe(&mut program_chars, Box::new(StdinByteIterator(stdin)), my_println as fn(String), MyOptions::from(matches) );
    while let Some(out) = main_iterator.next() {
        if ! (out.is_ascii_graphic() || out.is_ascii_whitespace()) { 
        print!("§{};",out);
        } else {
        print!("{}",char::from_u32(out as u32).unwrap());
        }
    }  
    println!(" -Program ended.");
    Ok(())
}

fn my_println(s: String) {
    println!("{}",s);
}

struct StdinByteIterator<'a>(StdinLock<'a>);
impl<'a> Iterator for StdinByteIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let mut byte = [0u8;1];
        let mut have_read_char = 0;
        while have_read_char == 0 {
            std::thread::sleep(Duration::new(0,1_000_000_000/60));
            have_read_char = self.0.read(&mut byte).unwrap_or(0); // when a char is read, this is 1.
        }
        Some(byte[0])
    }
}