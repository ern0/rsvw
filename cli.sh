#!/bin/bash
clear

function test {
	echo [rsvw "$@"]
	cargo run -- "$@" | cat -n
	echo ' '
}

echo filter
cat example.rsv | target/debug/rsvw | cat -n
echo ' '
test --line-starting '' -e '' -o '<value>' -c '</value>' -f '' -n '<null/>' example.rsv
test -s '<line>' -e '</line>' -o '<value>' -c '</value>' -f '' -n '<null/>' example.rsv
