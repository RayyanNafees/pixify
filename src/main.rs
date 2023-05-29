
mod convert;

fn main() {
    let args = convert::get_args();
    let bytes = convert::bytes_from_file(&args);
    println!("{bytes:?}")
}
