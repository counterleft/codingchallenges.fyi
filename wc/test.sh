#!/bin/bash

source './assert.sh/assert.sh'

assert_eq "  342190 test.txt" "$(elixir ccwc.ex -c test.txt)" "byte count"
assert_eq "    7145 test.txt" "$(elixir ccwc.ex -l test.txt)" "line count"
assert_eq "   58164 test.txt" "$(elixir ccwc.ex -w test.txt)" "word count"
assert_eq "  339292 test.txt" "$(elixir ccwc.ex -m test.txt)" "character count (locale dependent)"
assert_eq "    7145   58164  342190 test.txt" "$(elixir ccwc.ex test.txt)" "no options (defaults to -l -w -c)"

assert_eq "       1" "$(echo "hello" | elixir ccwc.ex -l)" "pipe - line hello"
assert_eq "  342190" "$(cat test.txt | elixir ccwc.ex -c)" "pipe - byte count test.txt"
assert_eq "    7145" "$(cat test.txt | elixir ccwc.ex -l)" "pipe - line count test.txt"
assert_eq "   58164" "$(cat test.txt | elixir ccwc.ex -w)" "pipe - word count test.txt"
assert_eq "  339292" "$(cat test.txt | elixir ccwc.ex -m)" "pipe - character test.txt"
