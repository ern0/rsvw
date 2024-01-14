#!/bin/bash
clear

function test {
	echo [cat-rsv $@]
	target/debug/cat-rsv $@ | cat -n
	echo ' '
}

test cli.sh

#test -s'[' -e']' -o'<' -c'>' -f'|' -n'null' example.rsv
cat example.rsv | target/debug/cat-rsv | cat -n

exit
test f1 f2 f3 - f4
test
test -n file
echo [cat-rsv $@]
cat example.rsv | target/debug/cat-rsv | cat -n
