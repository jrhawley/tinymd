use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use clap::{App, Arg};

fn parse_markdown_file(_file: &str, _outfile: &str) {
    // input file path
    let infile = Path::new(_file);
    // input file handle
    let file = File::open(&infile).expect("[ ERROR ] Failed to open input file.");

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of heading tags

    let mut tokens: Vec<String> = Vec::new(); // vector to store all tokens
    let reader = BufReader::new(file); // read line-by-line

    // iterate through lines
    for line in reader.lines() {
        let contents = line.unwrap();
        // parse first character from line
        let mut first_char: Vec<char> = contents.chars().take(1).collect();
        let mut outline = String::new();

        // see if the first character is a header line marker (i.e. '#')
        match first_char.pop() {
            Some('#') => {
                // close paragraph, if currently open
                if _ptag {
                    _ptag = false;
                    outline.push_str("</p>\n");
                }
                // if currently header, this line should be a new header
                if _htag {
                    _htag = false;
                    outline.push_str("</h1>\n");
                }

                // set _htag to true, write new header
                _htag = true;
                outline.push_str("\n<h1>");
                outline.push_str(&contents[2..]);
            },
            _ => {
                // close header, if currently open
                if _htag {
                    _htag = false;
                    outline.push_str("</h1>\n");
                }
                // if currently paragraph, continue
                if !_ptag {
                    _ptag = true;
                    outline.push_str("<p>");
                }

                // add paragraph contents
                outline.push_str(&contents);
            }
        }

        // close appropriate tags
        if _ptag {
            _ptag = false;
            outline.push_str("</p>\n");
        }
        // if currently header, this line should be a new header
        if _htag {
            _htag = false;
            outline.push_str("</h1>\n");
        }

        // check for empty lines, push to tokens
        if outline != "<p></p>\n" {
            tokens.push(outline);
        }
    }

    // print each token, either to stdout or to a file
    if !_outfile.is_empty() {
        let mut output_file = File::create(_outfile.to_string())
            .expect("[ ERROR ] Could not create output file!");
        for t in &tokens {
            output_file.write_all(t.as_bytes()).expect("[ ERROR ] Could not write to output file.")
        }
    } else {
        for t in &tokens {
            println!("{}", t);
        }
    }

}

fn main() {
    let _args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .about("Output file to write to")
            .required(false)
            .takes_value(true)
        )
        .arg(Arg::with_name("input")
            .about("Input markdown file to read")
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    // input file
    let input_file: String;
    // output file
    let output_file: String;

    if let Some(o) = _args.value_of("input") {
        input_file = String::from(o);
        if let Some(o) = _args.value_of("output") {
            output_file = String::from(o);
            parse_markdown_file(&input_file, &output_file);
        } else {
            parse_markdown_file(&input_file, "");
        }
    }
}