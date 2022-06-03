# Elastic dump

Helps you to query and write data to a file from Elasticsearch, Inspired by npm package [elasticdump](https://github.com/elasticsearch-dump/elasticsearch-dump#readme). 
Package supports signing request with AWS keys for authentication.

## Installation

Goto [release](https://github.com/saiumesh535/elastic-dump-rust/releases) and download latest version from assets, or build steps:

- need rust, see <https://rustup.rs/>
- `git clone https://github.com/saiumesh535/elastic-dump-rust.git`
- `cd elastic-dump-rust`
- `cargo install --path .`

## Usage

```cmd
elastic-dump-rust --help
```

```cmd
elastic-dump-rust 0.0.1
Sai Umesh <https://twitter.com/saiumesh>
Elasticsearch dump cli

USAGE:
    elastic-dump-rust <es_url> <index> <query> <file_name> <key_id> <secret_key> [region]

ARGS:
    <es_url>        Elasticsearch URL
    <index>         Name of the index
    <query>         ES Query
    <file_name>     Name of the file to be created
    <key_id>        AWS Access Key Id
    <secret_key>    AWS Secret Access Key
    <region>        AWS region [default: us-east-1]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Example

```cmd
elastic-dump-rust -- "<ES_URL>" "<Index_name>" "{\"size\":100,\"query\":{\"bool\":{\"filter\":{\"term\":{\"properties.appId\":928}}}}}" "<file_name>" "AWS_KEY_ID" "AWS_SECRET"
```