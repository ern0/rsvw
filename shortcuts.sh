#!/bin/bash

alias s="clear ; target/debug/cat-rsv example.rsv"

alias p="clear ; cargo build"
#alias p="clear ; cargo rustdoc -- --html-in-header doc/style.css"

#alias t="clear ; cargo test --lib -- --nocapture tst"
#alias t="clear ; cargo pretty-test --lib -- --nocapture"
alias t="clear ; cargo pretty-test main -- --nocapture"
