use std::fs::File;
extern crate libc;

fn main() {
    File::create(&format!("xx/{}", unsafe { libc::getpid() })).expect("create");
}
