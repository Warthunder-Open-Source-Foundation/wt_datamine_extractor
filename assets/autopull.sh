#!/bin/bash
cd .. &&
OLD=`cat meta_index/version.txt` &&
cd resources/cache &&
git pull &&
cd ../.. && 
cargo run --release && cargo test --release &&
NEW=`cat meta_index/version.txt` &&
semver_cli --cmp $OLD:$NEW --exit &&
git add . &&
git commit -m "some git message containing chron current time ans version if it changed" &&
git push
