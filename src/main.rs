use std::io::Write;

mod autocorrect;
use autocorrect::AutoCorrectionProvider;

fn main() {
    let ac = AutoCorrectionProvider::new();
    loop {
        println!();
        print!("stdin> ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let matches = ac.suggest(&buf.trim(), 5, 500);
        println!(
            "match> {} [{}]",
            matches.best_match(),
            matches
                .get_suggestions()
                .iter()
                .map(|s| format!("{}: {}", s.word, s.priority))
                .collect::<Vec<String>>()
                .join(", ")
        );
        std::io::stdout().flush().unwrap();
    }
}
