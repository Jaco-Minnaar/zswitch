#!/bin/sh

wget https://github.com/nushell/nushell/releases/download/0.101.0/nu-0.101.0-x86_64-unknown-linux-gnu.tar.gz
tar -xvf nu-0.101.0-x86_64-unknown-linux-gnu.tar.gz

cp nu-0.101.0-x86_64-unknown-linux-gnu/nu $HOME/.local/bin
rm -rf nu-0.101.0-x86_64-unknown-linux-gnu
rm -f nu-0.101.0-x86_64-unknown-linux-gnu.tar.gz
