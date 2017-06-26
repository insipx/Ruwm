#!/bin/bash

cargo build &&
Xephyr :1 &
DISPLAY=:1 ./target/debug/ruwm
