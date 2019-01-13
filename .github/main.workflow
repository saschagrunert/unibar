workflow "Workflow" {
  on = "push"
  resolves = ["Build"]
}

action "Build" {
  uses = "saschagrunert/build-rust@latest"
  runs = "make"
}
