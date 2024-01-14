#!/bin/bash
clear

function test {
	echo [rsvw $@]
	target/debug/rsvw $@ | cat -n
	echo ' '
}

test cli.sh

test -s'<line>' -e'</line>' -o'<field>' -c'</field>' -f -n'<null/>' example.rsv

exit
test f1 f2 f3 - f4
test 
test -n file
echo [rsvw $@]
cat example.rsv | target/debug/rsvw | cat -n
