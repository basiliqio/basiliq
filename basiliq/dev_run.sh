#!/bin/bash
pkill basiliq
cargo r -- serve -P 4444 basiliq_config.yaml &

