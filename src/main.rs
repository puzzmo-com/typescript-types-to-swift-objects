mod lib; 

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    lib::run_for_ts_file_path(path); // Use the function from lib.rs
}