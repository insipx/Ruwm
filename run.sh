#!/bin/bash
# this is supposedto work
# but mostly it doesn't, andjust produces an error
cargo build &&
Xephyr :1 -resizeable &
DISPLAY=:1 ./target/debug/ruwm
