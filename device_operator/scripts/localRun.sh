#!bash

RUSTFLAGS="-Z threads=8" LISTENER=websocket cargo +nightly watch -x run
