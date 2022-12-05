#!/bin/bash
cargo new --vcs=none --bin --name aoc2022-$1 $1
cp 01/Makefile $1/Makefile
mkdir $1/src/bin
mv $1/src/main.rs $1/src/bin/part_one.rs

