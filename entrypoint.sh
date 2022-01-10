#!/bin/sh
while [ 1 ];
do
    /usr/local/cargo/bin/diesel database setup && break;
done
./target/release/torimies-rs