# TODOTimeTracker
A to-do list and tracker for time spent per task. With a lightweight backend RESTful server written completely in Rust and frontend with vanilla JS and WASM.

## Progress:
- [x] PostgreSQL database running in a docker container
- [x] RESTful backend server in rust
- [ ] frontend with vanilla JS, HTML, CSS, and WASM for logic

## Future plans:
- [ ] move full backend to alpine-based docker containers for efficiency; a nice to have but not a necessity 

## How to run a demo of the backend:
 1. use docker compose to run the file in dockerfiles directory (pulls down files for a postgres 16 release tag docker container)
 2. use `cargo run --release` to compile and run (requires cargo)
 3. use `curl` to test out the api (docs currently unavailable. Check source code for endpoint addresses)

This will be a remake of an older project (a full functioning project): [Project Jasmine](https://github.com/TheAbyssBr0/ProjectJasmine)
