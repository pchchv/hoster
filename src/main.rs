use libc::exit;

pub mod ops;
mod options;

pub struct Error(pub String);

fn actual() -> i32 {
    if let Err(err) = server() {
        eprintln!("{}", err.0);
        1
    } else {
        0
    }
}

fn server() -> Result<(), Error> {
    Ok(())
}

fn main() {
    let result = actual();
    unsafe { exit(result) }
}