use lib::lines::Uniq;

fn main() {
    let input = std::fs::read_to_string("countries.txt").expect("failed to read input file");
    let unique_finder = Uniq::new(&input);
    let res = unique_finder.get_uniq_lines();
    println!("{:#?}\n\nLength = {}", res, res.len());
}
