#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/debug/ecc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 21 '5+20-4'
assert 41 ' 12 + 34 -5 '
assert 36 '1+2+3+4+5+6+7+8'

echo OK
