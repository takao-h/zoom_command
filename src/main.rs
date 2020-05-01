use std::process;

fn main() {
    let result = run();

    match result {
        Err(error) => {
            let stderr = srd::io::stderr();
            default_error _handler(&error, &mut stderr.lock());
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        },
        Ok(true) => {
            process::exit(0);
        },
    }
}
