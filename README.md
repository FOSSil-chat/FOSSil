# FOSSil Chat

## What is it?
FOSSil Chat is a work-in-progress, privacy-focused chat platform featuring community-hosted, federated servers, end-to-end encryption (E2EE), and a user-friendly experience.
---
## Why does it exist?
It exists to address a simple, yet important and often-overlooked problem - how can people communicate privately in an internet environment where control and surveillance are increasing?
---
## What do we have planned?
Currently, we are under early development, with little code and functionality, but our aim is to create the most privacy-oriented and simple-to-use IM program. This means:
- Extremely strong E2EE
- Markdown support for messages
- Fast messaging times
- Federated servers (community-hosted)
---
## Getting Started
- You need Rust and Cargo
- To clone the repository, ensure you have Git installed, and type `git clone https://codeberg.org/FOSSil/FOSSil.git` into the terminal.
- Then, `cd` into the cloned directory, and type "cargo run" to run the program.
---
## Project Structure
- [docs/](https://codeberg.org/FOSSil/FOSSil/src/branch/main/docs) contains documentation for the project.
- [src/](https://codeberg.org/FOSSil/FOSSil/src/branch/main/src) contains [src/main.rs](https://codeberg.org/FOSSil/FOSSil/src/branch/main/src/main.rs) (the main file launching the program), [src/packet.rs](https://codeberg.org/FOSSil/FOSSil/src/branch/main/src/packet.rs) (the file containing the Packet enum), [src/handler.rs](https://codeberg.org/FOSSil/FOSSil/src/branch/main/src/handler.rs) (the packet handler), and [src/server.rs](https://codeberg.org/FOSSil/FOSSil/src/branch/main/src/server.rs) (containing the ServerState struct).
- [Project Root](https://codeberg.org/FOSSil/FOSSil/src/branch/main) contains [Cargo.lock](https://codeberg.org/FOSSil/FOSSil/src/branch/main/Cargo.lock) and [.gitignore](https://codeberg.org/FOSSil/FOSSil/src/branch/main/.gitignore), which are unrelated to the program, [Cargo.toml](https://codeberg.org/FOSSil/FOSSil/src/branch/main/Cargo.toml), which is the Cargo config, [README.md](https://codeberg.org/FOSSil/FOSSil/src/branch/main/README.md), which is this file, [MAINTAINERS.md](https://codeberg.org/FOSSil/FOSSil/src/branch/main/MAINTAINERS.md), containing the names of the FOSSil Project maintainers, [CONTRIBUTING.md](https://codeberg.org/FOSSil/FOSSil/src/branch/main/CONTRIBUTING.md), which contains instructions on the format of writing code in this codebase, [LICENSE](https://codeberg.org/FOSSil/FOSSil/src/branch/main/LICENSE) which contains the GNU Affero General Public License v3.0, and [TRADEMARKS.md](https://codeberg.org/FOSSil/FOSSil/src/branch/main/TRADEMARKS.md), which contains trademark information.

> [!NOTE]
> The project is in very early development, so there is no messaging system in place yet, but rather a program skeleton.

> [!WARNING]
> FOSSil is currently under active development and is not yet ready for production use. Features, protocols, and APIs may change without notice.

This project is licensed under the GNU AGPL-3.0 License.