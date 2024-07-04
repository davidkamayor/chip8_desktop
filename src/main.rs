fn main() {
    let args: Vec<_> = env::args().collect();
    if args.leng() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }
}
