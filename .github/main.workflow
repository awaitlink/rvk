workflow "fmt check, clippy, test" {
  on = "push"
  resolves = ["test"]
}

action "fmt check" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt --  --check"
}

action "clippy" {
  uses = "icepuma/rust-action@master"
  needs = ["fmt check"]
  args = "cargo clippy -- -Dwarnings"
}

action "test" {
  uses = "icepuma/rust-action@master"
  needs = ["clippy"]
  args = "cargo test"
}
