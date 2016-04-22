#!/bin/bash

set -e
set -u
set -o pipefail

rm -rf /etc/rustyweb/
cp .default_etc_directory /etc/rustyweb -r
cargo build
