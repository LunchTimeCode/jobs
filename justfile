import 'linting.just'
import 'docker.just'

image_name := "ghcr.io/lunchtimecode/jobs"


run *args:
    cargo run -- {{args}}


w:
    cargo watch --ignore 'assets/css' -s 'just run'
