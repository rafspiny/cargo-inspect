//! `cargo inspect`
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate cargo_inspect;

extern crate structopt;

use cargo_inspect::inspect;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Opt {
    #[structopt(name = "inspect")]
    Inspect {
        /// Input file
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        input: PathBuf,

        /// rustc "unpretty" parameters
        #[structopt(long = "unpretty", default_value = "hir")]
        unpretty: String,

        /// Theme for syntax highlighting. See syntex documentation for options
        #[structopt(long = "theme", default_value = "base16-ocean.dark")]
        theme: String,
    },
}

fn main() {
    let Opt::Inspect {
        input,
        unpretty,
        theme: _, // TODO: pass through theme
    } = Opt::from_args();
    inspect(input, unpretty).expect("Cannot print result")
}
