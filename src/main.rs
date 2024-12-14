use libc::exit;


fn main() {
    let result = actual();
    unsafe { exit(result) }
}

fn actual() -> i32 {
    1
}