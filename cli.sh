#!/bin/bash
clear

function test {
	echo [rsvw $@]
	target/debug/rsvw $@ | cat -n
	echo ' '
}

echo filter
cat example.rsv | target/debug/rsvw | cat -n
echo ' '
test --line-start=""  -e'' -o'<value>' -c'</value>' -f"" -n'<null/>' example.rsv
test -s "<line>" -e'</line>' -o'<value>' -c'</value>' -f"" -n'<null/>' example.rsv
