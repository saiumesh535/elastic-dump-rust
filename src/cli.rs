use clap::{Arg, ArgMatches, Command};

pub fn init_args() -> ArgMatches {
    let command = Command::new("elastic-dump-rust")
        .about("Elasticsearch dump cli")
        .version("0.0.1")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("Sai Umesh <https://twitter.com/saiumesh>")
        .arg(
            Arg::new("es_url")
                .required(true)
                .takes_value(true)
                .help("Elasticsearch URL"),
        )
        .arg(
            Arg::new("index")
                .required(true)
                .takes_value(true)
                .help("Name of the index"),
        )
        .arg(
            Arg::new("query")
                .required(true)
                .takes_value(true)
                .help("ES Query"),
        )
        .arg(
            Arg::new("file_name")
                .required(true)
                .takes_value(true)
                .help("Name of the file to be created"),
        )
        .arg(
            Arg::new("key_id")
                .required(true)
                .takes_value(true)
                .help("AWS Access Key Id"),
        )
        .arg(
            Arg::new("secret_key")
                .required(true)
                .takes_value(true)
                .help("AWS Secret Access Key"),
        )
        .arg(
            Arg::new("region")
                // .required(true)
                // .takes_value(true)
                .default_value("us-east-1")
                .help("AWS region"),
        );
    command.get_matches()
}
