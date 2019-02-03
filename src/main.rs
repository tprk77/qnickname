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

mod qnicknames;

use crate::qnicknames::QHashMap;

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
    let name = prompt_name()?;
    // It's guaranteed to be non-empty
    let first_char = name.chars().flat_map(|c| c.to_lowercase()).nth(0).unwrap();
    let remaining_chars = name.chars().flat_map(|c| c.to_lowercase())
        .skip(1).take(20).collect::<Vec<char>>();
    // Use the first character to get the group of possible QNickNames
    if let Some(qvec) = QHashMap.get(&first_char) {
        // Use the remaining characters for an offset
        let offset = remaining_chars.iter().map(|c| *c as usize).sum::<usize>();
        let qname = qvec.get(offset % qvec.len()).unwrap();
        println!("Your QNickName is: {}", qname);
    } else {
        // This may happen if we get a weird character
        println!("You don't have a name! (Sorry!)");
    }
    Ok(())
}

fn main() {
    if let Err(error) = qmain() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        std::process::exit(1);
    }
}
