#!/bin/bash
cd .. &&
git pull &&
OLD=$(cat meta_index/version.txt) &&
cd resources/cache &&
GIT_OUT=$(git pull 2>&1) &&
if [ "$GIT_OUT"  != "Already up to date." ]; then
  cd ../.. &&
  cargo run --release && cargo test --release &&
  NEW=$(cat meta_index/version.txt) &&
  semver_cli --cmp $OLD:$NEW --exit &&
  git add . &&
  git commit -m "automatically update to ${NEW}" &&
  git push
fi
echo "Git output was up to date or failed otherwise"