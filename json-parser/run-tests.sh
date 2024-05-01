#!/bin/bash

source './assert.sh/assert.sh'

actual="$(cat tests/step1/invalid.json | elixir json-parser.ex)"
assert_eq 1 "$?" "step1 invalid"

actual="$(cat tests/step1/valid.json | elixir json-parser.ex)"
assert_eq 0 "$?" "step1 valid"

actual="$(cat tests/step2/valid.json | elixir json-parser.ex)"
assert_eq 0 "$?" "step2 valid"

actual="$(cat tests/step2/valid2.json | elixir json-parser.ex)"
assert_eq 0 "$?" "step2 valid2"

actual="$(cat tests/step2/invalid.json | elixir json-parser.ex)"
assert_eq 1 "$?" "step2 invalid"
#
# actual="$(cat tests/step2/invalid2.json | elixir json-parser.ex)"
# assert_eq 1 "$?" "step2 invalid2"
