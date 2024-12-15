use libc::exit;

pub struct Error(pub String);

fn main() {
    let result = actual();
    unsafe { exit(result) }
}

fn actual() -> i32 {
    1
}

fn server() -> Result<(), Error> {
    Ok(())
}