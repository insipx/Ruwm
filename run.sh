#!/bin/bash

cargo build &&
Xephyr :1 -resizeable &
DISPLAY=:1 ./target/debug/ruwm &
