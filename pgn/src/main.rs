use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut inp: String = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();

    let inp = inp.trim();

    println!(
        "Trying to open file: {inp} from {:?}",
        env::current_dir().unwrap()
    );

    let path = Path::new(&inp);

    // let path3 = path::Path::new("wtf2.txt");
    println!("{}", path.display());
    let contents: String = fs::read_to_string(path).expect("Failed to open file");
    println!("{:?}", contents);

    Ok(())
}
