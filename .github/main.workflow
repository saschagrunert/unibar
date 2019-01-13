workflow "Workflow" {
  on = "push"
  resolves = ["Build"]
}

action "Build" {
  uses = "docker://saschagrunert/build-rust:latest"
  runs = "make"
}
