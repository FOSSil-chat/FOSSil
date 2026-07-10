# Development - How to Contribute
## Install Rust
Run this in a terminal:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
This will install the full Rust toolchain (click [here](https://rust-lang.org/tools/install/) for more information).
## Clone Repo
Run this in a terminal:
`git clone https://codeberg.org/FOSSil/FOSSil-Chat`
Then, run this:
`cd FOSSil-Chat`
Now, you are in your **cloned repo**.
## Run the program
Run this in a terminal:
`cd server`
`cargo run`
to run the server, or:
`cd client`
`cargo run`
to run the client.
## Contributing
All names must follow **Rust naming conventions**.
First, you must switch to a **new branch**:
`git switch -u branchname`

Run:
`cargo fmt`
`cargo clippy` (must run/pass without warning/error)
`cargo test` (no tests can fail)
Then, if the code is ready to push, run these commands:
`git add .`
`git commit -m "commit name"`
`git push origin branchname`

Then, go to Codeberg and open a pull request to merge your branch with the main branch.