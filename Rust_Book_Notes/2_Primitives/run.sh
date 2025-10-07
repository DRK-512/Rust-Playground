#!/bin/sh
#source /home/dev/.cargo/env
rustc primitives.rs
./primitives
rm primitives
