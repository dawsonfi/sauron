<p align="center">
    <img align="left" src="https://raw.githubusercontent.com/dawsonfi/sauron/main/dev/resources/logo.png" height="100px" alt="Sauron Eye">
</p>

# Sauron

Cloudwatch CLI Tool

<br />


[![Current Crates.io Version](https://img.shields.io/crates/v/cw-sauron.svg)](https://crates.io/crates/cw-sauron)
![Release workflow](https://github.com/dawsonfi/sauron/actions/workflows/rust.yml/badge.svg)

# Installation

In order to install this cli, run the following command:

`cargo install cw-sauron`

# Usage

Invoke the cli using the `cw-sauron --help` command to see the available commands.

## AWS Configuration

sauron fetches the aws configuration from the `~/.aws/credentials` file, which should contain the following data:

* `region`
* `aws_access_key_id`
* `aws_secret_access_key`

for instructions see [AWS Configuration and credential file settings](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)

## List Queries

run `cw-sauron list_queries` to list the available queries on the configured aws account.