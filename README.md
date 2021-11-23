# Antecedence

> ### The act of going before

Antecedence is a small application for organising progress of tasks in a project. This is done via a
number of estimation steps.

1. Break the project up into multiple tasks
2. Clients can provide estimations on the value the completion of those tasks will provide
3. Team members estimate the time to complete those tasks
4. Client advocates are team members who can act on the clients' behalf and can estimate both

With this information the tasks will be organised in an order to attempt to maximise rate of value
provided as early as possible. Tasks will generally be recommened to team members who put lower
estimates for that task, assuming that their lower estimate represents personally more proficiency
in completing said task.

## Building and Running

This project is primarily organized around Cargo in [Rust](https://www.rust-lang.org/).

1. Get the Rust toolchain via [Rustup](http://rustup.rs)
2. Build the debug version `cargo build` or run with `cargo run`
3. For production add the `--release` flag, `cargo {build|run} --release`

Logging is done via `stdout`
