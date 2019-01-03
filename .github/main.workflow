workflow "New workflow" {
  on = "push"
  resolves = ["docker://rust"]
}

action "docker://rust" {
  uses = "docker://rust"
  args = "-v \"$PWD\":/usr/src/rust-lsm -w /usr/src/rust-lsm rust:1.23 cargo build --release"
}
