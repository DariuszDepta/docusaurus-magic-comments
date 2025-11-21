use docusaurus_magic_comments::{common, diff};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} FILE START_LINE END_LINE", args[0]);
        eprintln!("Example: {} introduction.md 86 107", args[0]);
        std::process::exit(1);
    }
    let start_line_number = args[2].parse::<usize>().expect("invalid start line number");
    let end_line_number = args[3].parse::<usize>().expect("invalid end line number");
    let lines = common::select_lines(
        common::load_file(&args[1]),
        start_line_number - 1,
        end_line_number - 1,
    );
    common::print_lines(diff::apply(lines));
}
