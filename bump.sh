#!/usr/bin/env bash

FROM=$1
TO=$2

if ! grep "version = \"$FROM\"" Cargo.toml &> /dev/null; then
	echo "\"$FROM\" doesn't seem to be a valid version."
	exit 1
fi

sed -i "s/version = \"$FROM\"/version = \"$TO\"/" Cargo.toml
sed -i "s/version = \"$FROM\"/version = \"$TO\"/" aocf_cli/Cargo.toml

cargo build || exit
cd aocf_cli
cargo build || exit

# requires `git-journal` -> `cargo install git-journal`
#git journal -o CHANGELOG.md

cd ..
git commit Cargo.toml aocf_cli/Cargo.toml aocf_cli/Cargo.lock CHANGELOG.md -m "Bump v$FROM → v$TO"
git tag "v$TO"
