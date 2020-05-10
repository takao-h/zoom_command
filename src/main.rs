// use std::process;

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
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    print!("{}", input);
}
