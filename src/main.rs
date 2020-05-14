// use std::process;
use structopt::{clap, StructOpt};


#[derive(Debug, StructOpt)]
#[structopt(name = "zoom_command")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(short = "n")]
    pub datetime: Option<usize>,

    #[structopt(
        short = "t",
        long = "threads",
        default_value(&THREADS),
        value_name = "NUM"
    )]
    pub threads: usize,

}

lazy_static! {
    static ref THREADS: String = format!("{}", num_cpus::get());
}

fn main() {
    // let result = run();

    // match result {
    //     Err(error) => {
    //         let stderr = srd::io::stderr();
    //         default_error _handler(&error, &mut stderr.lock());
    //         process::exit(1);
    //     }
    //     Ok(false) => {
    //         process::exit(1);
    //     },
    //     Ok(true) => {
    //         process::exit(0);
    //     },
    // }
    let _ = Opt::from_args();
}
