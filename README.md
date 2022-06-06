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
elastic-dump-rust 0.0.3
Sai Umesh <https://twitter.com/saiumesh>
Elasticsearch dump cli

USAGE:
    elastic-dump-rust [OPTIONS] --es_url <es_url> --index <index> --query <query> --file_name <file_name> --key <key_id> --secret <secret_key>

OPTIONS:
    -e, --es_url <es_url>          Elasticsearch URL
    -f, --file_name <file_name>    Name of the file to be created
    -h, --help                     Print help information
    -i, --index <index>            Name of the index
    -k, --key <key_id>             AWS Access Key Id
    -q, --query <query>            ES Query
    -r, --region <region>          AWS region [default: us-east-1]
    -s, --secret <secret_key>      AWS Secret Access Key
    -V, --version                  Print version information
```

## Example

```cmd
elastic-dump-rust --es_url "<es_url>" \
--index "index_name" \
--query "<query>" \
--file_name "<file_name>" \
--key "<AWS_KEY_ID>" \
--secret "<AWS_SECRET>"
```