use cli::Args;

fn main() {
    let args = Args::get_args();
    println!("{:#?}", args);
}
