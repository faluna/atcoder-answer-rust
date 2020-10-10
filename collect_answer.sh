#!/usr/local/bin/zsh

PROJECT_DIR="${HOME}/rust-project"
COLLECT_DIR="${HOME}/rust-project/atcoder-answer-rust"

directory_list=(`\ls -1 "$HOME/rust-project" | rg -e "a[blgrt][cl]\d{2,3}"`)

for dir in $directory_list; do
  target_dir="$COLLECT_DIR/$dir"
  mkdir -p $target_dir
  cp -r $PROJECT_DIR/$dir/src $target_dir/
done
