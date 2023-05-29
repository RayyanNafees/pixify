
mod conv;

fn main() {
    let args = conv::get_args();
    let bytes = conv::bytes_from_file(&args);
    println!("{bytes:?}")
}
