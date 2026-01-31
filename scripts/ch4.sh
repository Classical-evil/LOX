#!/bin/bash
RUSTFLAGS="-A warnings" cargo test -- --nocapture -q
