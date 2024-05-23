# zero-cli
A command line tool for Zero Secrets Manager.


## Overview
This CLI tool allows users to manage secrets and projects within the Zero Secrets Manager. It provides functionalities for authentication, project management, secret management, team handling, and more.


## Installation

To install the `zero-cli` from Crates.io, use the following command:

```bash
cargo install zero-cli
```

This command will download the crate, compile all the binary targets it contains (in "release" mode), and copy them into the ~/.cargo/bin/ directory. Ensure that your shell knows to look there for binaries!


## Usage

The CLI offers various subcommands:

- auth: Authentication and authorization functionalities.
- projects: Manage projects, including creation, deletion, editing, listing, sharing, usage overview, and viewing details.
- secrets: Handle secrets, allowing users to create, delete, edit, list, share, drop, and view details.
- teams: Operations related to teams, such as creation, deletion, editing, leaving, listing, managing users, and viewing details.
- tokens: Token management including creation, deletion, list view and regeneration.
- help: Display help information about the CLI or a specific subcommand.

## 🛠 Development guide

Run
```bash
cargo run
```
or if you want to test the command run from src folder
```bash
cargo run -- auth login
```

Codegen
```bash
cargo install graphql_client_cli
```
```bash
graphql-client generate <query_path> --schema-path <schema_path>
````

as example
```
graphql-client generate src/auth/graphql/me.graphql --schema-path schema.graphql
```
