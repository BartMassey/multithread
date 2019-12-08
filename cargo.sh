#!/bin/sh

DEBUG=$1
case $DEBUG in
    true) NAME="debug" ;;
    false) NAME="ndebug" ;;
esac

sed -e "s/NAME/$NAME/g" -e "s/DEBUG/$DEBUG/g" <Cargo.toml.in >Cargo.toml
rm -rf target/
cargo build --release
mv -f target/release/$NAME .
