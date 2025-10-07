#!/bin/sh
#source /home/dev/.cargo/env
rustc custom-types.rs
./custom-types
rm custom-types
