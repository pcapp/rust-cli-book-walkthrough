fn main() {
    for (index, arg) in std::env::args().enumerate() {
        println!("[{index}] {arg}")
    }
}
