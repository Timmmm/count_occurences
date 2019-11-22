
use aho_corasick::AhoCorasick;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: ./count_occurances 'pattern' <filename>");
        return;
    }

    let f = File::open(&args[2]).expect("Error opening input file");
    let reader = BufReader::new(f);

    let patterns = &[&args[1]];
    let ac = AhoCorasick::new(patterns);
    let count = ac.stream_find_iter(reader).count();

    println!("{}", count);
}
