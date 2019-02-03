// Copyright (c) 2019 Tim Perkins

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io::{self, Write};
use getopts::Options;

mod qnicknames;

use crate::qnicknames::get_qnickname;

fn prompt_name() -> io::Result<String> {
    let mut input_str = String::new();
    while input_str.is_empty() {
        print!("Please enter your name: ");
        io::stdout().flush()?;
        let stdin = io::stdin();
        stdin.read_line(&mut input_str)?;
        // Better way to do this?
        input_str = input_str.trim().to_string();
    }
    Ok(input_str)
}

fn qmain() -> io::Result<()> {
    // It's guaranteed to be non-empty
    let name = prompt_name()?;
    // Get the actual qnickname
    if let Some(qnickname) = get_qnickname(&name) {
        // Use the remaining characters for an offset
        println!("Your QNickName is: {}", qnickname);
    } else {
        // This may happen if we get a weird character
        println!("You don't have a name! (Sorry!)");
    }
    Ok(())
}

fn print_usage_and_exit(program: &str, opts: &Options) -> ! {
    let _ = writeln!(io::stderr(), "{}", opts.usage(&opts.short_usage(&program)));
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(error) => {
            let _ = writeln!(io::stderr(), "Error: {}", error);
            print_usage_and_exit(&program, &opts);
        }
    };
    if opt_matches.free.len() > 0 || opt_matches.opt_present("h") {
        print_usage_and_exit(&program, &opts);
    }
    if let Err(error) = qmain() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        std::process::exit(1);
    }
}
