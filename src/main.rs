#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate git2;

use docopt::Docopt;
// use std::env;

docopt!(Args derive Debug, "
Usage:
  shish --help
  shish git-pwd [--only=<level>]

Options:
  -h, --help        Show this message.
  --only=<level>    Show only pwd at specified level.
                    Valid values: inside, outside, root
", flag_only: Option<Level>);

#[derive(Debug, RustcDecodable)]
enum Level { Inside, Outside, Root }

// #[derive(Debug, RustcDecodable)]
// struct Args {
//     cmd_git_pwd: bool,
//     flag_inside: bool,
//     flag_outside: bool,
//     flag_root: bool
// }

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    // let args: Args = Docopt::new(USAGE)
    //     .and_then(|d| d.decode())
    //     .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
// fn do_work(inp: &str, out: Option<String>) {
//     println!("{}", inp);
//     match out {
//         Some(x) => println!("{}", x),
//         None => println!("No Output"),
//     }
// }

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let program = args[0].clone();

//     let mut opts = Options::new();
//     opts.optopt("o", "", "set output file name", "NAME");
//     opts.optflag("h", "help", "print this help menu");
//     let matches = match opts.parse(&args[1..]) {
//         Ok(m) => { m }
//         Err(f) => { panic!(f.to_string()) }
//     };
//     if matches.opt_present("h") {
//         print_usage(&program, opts);
//         return;
//     }
//     let output = matches.opt_str("o");
//     let input = if !matches.free.is_empty() {
//         matches.free[0].clone()
//     } else {
//         print_usage(&program, opts);
//         return;
//     };
//     do_work(&input, output);
// }
