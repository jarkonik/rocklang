#!/bin/bash

cargo watch -w src -w tests -x 'tarpaulin --ignore-tests   --output-dir target/tarpaulin -o Lcov'
