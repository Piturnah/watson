# Running the server

You will need to have a Postgresql database running. Once you have that, provide the DATABASE_URL environment variable in `.env`. Then you just have to use `cargo run` (with [Rust](https://www.rust-lang.org/) installed on your system).

## Password Security

Passwords are hashed using [argon2id](https://en.wikipedia.org/wiki/Argon2) via the [argon2](https://crates.io/crates/argon2) crate in accordance with advice found [here](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html).
