use getopts::{Options, Result, Matches};

pub fn set_opts(args: Vec<String>) -> Result {
    let mut opts = Options::new();
    opts.optopt("i", "input", "Chip8 Program file to run", "File");
    opts.parse(args)
}

pub fn get_filename(matches: &Matches) -> String {
     match matches.opt_str("i") {
        Some(f) => f,
        None => panic!("File name required to run emulators")
    }
}