use clap::{Parser, arg};
use core::panic;
use itertools::Itertools;
use std::io::Read;

#[derive(Debug, Parser)]
struct Args {
    /// Amount of elements in one permutation. Zero implies element count
    #[arg(long, short, default_value_t = 0)]
    k: usize,

    /// Delimiter between input elements, read from stdin
    #[arg(long, short, default_value = "\n")]
    delim: String,

    /// Separator between elements in a permutation
    #[arg(long, short, default_value = "")]
    elem_separator: String,

    /// Separator between permutations
    #[arg(long, short, default_value = "\n")]
    separator: String,
}

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL)
    };

    let mut args = Args::parse();

    let mut inp = String::new();
    std::io::stdin().read_to_string(&mut inp).unwrap();

    inp = match inp.strip_prefix(&args.delim) {
        Some(s) => s.to_owned(),
        None => inp,
    };
    inp = match inp.strip_suffix(&args.delim) {
        Some(s) => s.to_owned(),
        None => inp,
    };

    inp = match inp.strip_suffix("\n") {
        Some(s) => s.to_owned(),
        None => inp,
    };

    let elements = inp
        .split(&args.delim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    if args.k == 0 {
        args.k = elements.len();
    } else if args.k > elements.len() {
        panic!(
            "requested k={}, but only got {} elements",
            args.k,
            elements.len()
        );
    }

    let mut first = true;

    // Apparently permutating an empty array with k=0, gives a single element.
    if !elements.is_empty() {
        elements
            .iter()
            .permutations(args.k)
            .map(|vec| vec.into_iter().join(&args.elem_separator))
            .for_each(|s| {
                if !first {
                    print!("{}", args.separator);
                }
                first = false;
                print!("{s}");
            });
    }

    if !first {
        println!();
    }
}
