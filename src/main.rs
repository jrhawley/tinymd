use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use clap::{App, Arg};

fn add_tag(_s: &str, _tag: &str, _open: bool) -> String {
    let mut new_s = String::from(_s);
    if _open {
        new_s.push_str("<");
        new_s.push_str(_tag);
        new_s.push_str(">");
    } else {
        new_s.push_str("</");
        new_s.push_str(_tag);
        new_s.push_str(">\n");
    }
    return new_s;
}

fn parse_markdown_line<'a>(contents: &str, mut _tag_open: bool, mut _tag: &'a str) -> (String, bool, &'a str) {
    // parse first character from line
    let mut first_char: Vec<char> = contents.chars().take(1).collect();
    let mut outline = String::new();

    // see if the first character is a header line marker (i.e. '#')
    match first_char.pop() {
        // if header line
        Some('#') => {
            // close previous tag, if currently open
            if _tag_open {
                _tag_open = false;
                outline = add_tag(&outline, _tag, _tag_open);
            }

            // set _htag to true, write new header
            _tag_open = true;
            _tag = "h1";
            outline = add_tag(&outline, _tag, _tag_open);
            outline.push_str(&contents[2..]);
        },
        // if part of the non-header text
        _ => {
            // close header, if currently open
            if _tag_open && _tag == "h1" {
                _tag_open = false;
                outline = add_tag(&outline, _tag, _tag_open);
            }
            // if currently paragraph, continue
            if !_tag_open {
                _tag_open = true;
                _tag = "p";
                outline = add_tag(&outline, _tag, _tag_open);
            }

            // add paragraph contents
            outline.push_str(&contents);
        }
    }

    // close appropriate tags
    if _tag_open {
        _tag_open = false;
        outline = add_tag(&outline, _tag, _tag_open);
    }

    return (outline, _tag_open, _tag);
}

fn parse_markdown_file(_file: &str, _outfile: &str) {
    let infile = Path::new(_file); // input file path
    // input file handle
    let file = File::open(&infile).expect("[ ERROR ] Failed to open input file.");
    let reader = BufReader::new(file); // read line-by-line

    let mut _tag_open: bool = false; // keep track of open tags
    let mut _tag = &""; // keep track of open tag type (initialize, but set to empty)
    let mut _outline: String; // parsed line contents to be written

    let mut tokens: Vec<String> = Vec::new(); // vector to store all tokens

    // iterate through lines
    for line in reader.lines() {
        let contents = line.unwrap();
        // parse the line of text
        let (mut _outline, mut _tag_open, mut _tag) = parse_markdown_line(&contents, _tag_open, _tag);
        // check for empty lines, push to tokens
        if _outline != "<p></p>\n" {
            tokens.push(_outline);
        }
    }

    // print each token, either to stdout or to a file
    if !_outfile.is_empty() {
        let mut output_file = File::create(_outfile.to_string())
            .expect("[ ERROR ] Could not create output file!");
        for t in &tokens {
            output_file.write_all(t.as_bytes()).expect("[ ERROR ] Could not write to output file.");
        }
    } else {
        for t in &tokens {
            println!("{}", t);
        }
    }
}

fn parse_markdown_stdin(_outfile: &str) {
    let _stdin = io::stdin(); // stdin
    let reader = _stdin.lock(); // input buffer; put into a separate variable to ensure io::stdin() isn't freed

    let mut _tag_open: bool = false; // keep track of open tags
    let mut _tag = &""; // keep track of open tag type (initialize, but set to empty)
    let mut _outline: String; // parsed line contents to be written

    let mut tokens: Vec<String> = Vec::new(); // vector to store all tokens

    // iterate through lines
    for line in reader.lines() {
        let contents = line.unwrap();
        // parse the line of text
        let (mut _outline, mut _tag_open, mut _tag) = parse_markdown_line(&contents, _tag_open, _tag);
        // check for empty lines, push to tokens
        if _outline != "<p></p>\n" {
            tokens.push(_outline);
        }
    }

    // print each token, either to stdout or to a file
    if !_outfile.is_empty() {
        let mut output_file = File::create(_outfile.to_string())
            .expect("[ ERROR ] Could not create output file!");
        for t in &tokens {
            output_file.write_all(t.as_bytes()).expect("[ ERROR ] Could not write to output file.");
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
            .required(false)
            .takes_value(true)
        )
        .get_matches();

    // parse arguments
    let input_file: String; // input file
    let output_file: String; // output file

    if let Some(o) = _args.value_of("input") {
        input_file = String::from(o);
    } else {
        input_file = "".to_string();
    }
    if let Some(o) = _args.value_of("output") {
        output_file = String::from(o);
    } else {
        output_file = "".to_string();
    }

    if input_file == "" {
        parse_markdown_stdin(&output_file);
    } else {
        parse_markdown_file(&input_file, &output_file);
    }
}
