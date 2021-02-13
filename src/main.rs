use clap::{App, Arg};
use plist::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("plist")
        .version("1.0")
        .author("Gerrit Birkeland <gerrit@gerritbirkeland.com>")
        .about("Converts plist files between binary and xml format")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .index(2),
        )
        .arg(
            Arg::with_name("binary")
                .short("b")
                .help("If set, convert to binary, otherwise, convert to XML"),
        )
        .get_matches();

    let in_file = matches.value_of("INPUT").unwrap();
    let out_file = matches.value_of("OUTPUT").unwrap_or(in_file);
    let file = Value::from_file(in_file)?;

    if matches.is_present("binary") {
        file.to_file_binary(out_file)?;
    } else {
        file.to_file_xml(out_file)?;
    }

    Ok(())
}
