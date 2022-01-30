use std::env;

use calm_io::{pipefail, stdoutln};

#[pipefail]
fn main() -> std::io::Result<()> {
    let arguments = env::args().skip(1);
    let arguments =
        if arguments.len() == 0 { vec![String::from("ouai")] } else { arguments.collect() };

    loop {
        for argument in &arguments {
            for (i, c) in argument.char_indices() {
                let i = i + c.len_utf8();
                let s = &argument[..i];
                stdoutln!("{}", s)?;
            }
        }
    }
}
