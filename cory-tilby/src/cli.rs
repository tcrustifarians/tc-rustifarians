use document;
use search;
use getopts::Options;
use std::env;

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [search/create]", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("s", "search", "Search documents");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("s") {
        let needle = matches.free[0].clone();
        search::for_relevant(&needle); 
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    document::create(&input);
}
