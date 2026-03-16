  - For static-site crates and any `ssg_whiz` site builds, always run the binary from the site crate directory, not the workspace root. `ssg_whiz` currently resolves `dist/`,
  `assets/`, and `content/` relative to the process working directory. Example: use `workdir=/workspace/crates/example-site` with `cargo run --bin example-site` or `cd /
  workspace/crates/example-site && cargo run --bin example-site`. Do not use `cargo run -p <site>` from `/workspace` unless the site is updated to use absolute crate-root paths.
