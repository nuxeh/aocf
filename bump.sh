#!/usr/bin/env bash

FROM=$1
TO=$2

if ! grep "version = \"$FROM\"" Cargo.toml &> /dev/null; then
	echo "\"$FROM\" doesn't seem to be a valid version."
	exit 1
fi

sed -i "s/version = \"$FROM\"/version = \"$TO\"/" Cargo.toml
sed -i "s/version = \"$FROM\"/version = \"$TO\"/" aocf_cli/Cargo.toml

cargo build
cd aocf_cli
cargo build

git ct Cargo.toml aocf_cli/Cargo.* -m "Bump $FROM → $TO"
