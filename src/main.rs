use std::io::Write;

use clap::{Command, Arg, ArgMatches};

fn main() -> Result<(), String> {
    loop {
        let line = read_line()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match process_command(line) {
            Ok(should_quit) => {
                if should_quit {
                    break;
                }
            }
            Err(err) => {
                log_err(err)?;
            }
        }
    }

    Ok(())
}

fn process_command(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(&args)
        .map_err(|e| e.to_string())?;

    let should_quit = handle_command(matches)?;

    Ok(should_quit)
}

fn handle_command(matches: ArgMatches) -> Result<bool, String> {
    match matches.subcommand() {
        Some(("select", sub_matches)) => {
            let ext_args = parse_ext_args(sub_matches, None);

            println!("{:?}", ext_args);
        },
        Some(("insert", sub_matches)) => {
            let ext_args = parse_ext_args(sub_matches, None);

            println!("{:?}", ext_args);
        },
        Some(("quit", _sub_matches)) => {
            write!(std::io::stdout(), "exiting...\n").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        },
        Some((name, _matches)) => unimplemented!("{}", name),
        None => unreachable!("subcommand required")
    }

    Ok(false)
}

/// Instantiates a SimpleDB command line interface.
fn cli() -> Command<'static> {
    Command::new("simpledb")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("select")
                .about("Query data from SimpleDB")
                .arg(
                    Arg::new("ext")
                    .takes_value(true)
                    .multiple_values(true)
                )
        )
        .subcommand(
            Command::new("insert")
                .about("Insert data into SimpleDB")
                .arg(
                    Arg::new("ext")
                    .takes_value(true)
                    .multiple_values(true)
                )
        )
        .subcommand(
            Command::new("quit")
                .alias("q")
                .about("Quit SimpleDB")
        )
}

/// Consumes user input via stdin, returns the buffer.
fn read_line() -> Result<String, String> {
    write!(std::io::stdout(), "simpledb > ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn log_err(err: String) -> Result<(), String> {
    write!(std::io::stdout(), "{}", err).map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;

    Ok(())
}

/// Parses the extra (trailing) arguments from a subcommand.
fn parse_ext_args(sub_matches: &ArgMatches, ext_args_name: Option<String>) -> Vec<&str> {
    const DEFAULT_EXT_ARGS_NAME: &str = "ext";
    let ext_args_name = ext_args_name.unwrap_or_else(|| String::from(DEFAULT_EXT_ARGS_NAME));
    let ext_args: Vec<&str> = match sub_matches.get_many::<String>(&ext_args_name) {
        Some(iter) => iter.map(|s| s.as_str()).collect(),
        None => vec![]
    };

    ext_args
}
