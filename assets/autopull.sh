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
git commit -m "automatically update to ${NEW}" &&
git push
