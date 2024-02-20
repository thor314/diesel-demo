# diesel-demo
![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink)
![](https://github.com/thor314/diesel-demo/actions/workflows/ci.yml/badge.svg)
<!-- [![crates.io](https://img.shields.io/crates/v/diesel-demo.svg)](https://crates.io/crates/diesel-demo) -->
<!-- [![Documentation](https://docs.rs/diesel-demo/badge.svg)](https://docs.rs/diesel-demo) -->
## License
Licensed under your option of either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
## Deploy
```sh
# run locally
cargo shuttle run
# deploy
cargo shuttle project start # only needed the first time
cargo shuttle deploy
```
## Project created with flags:
- project-name: diesel-demo
- description:  diesel-demo
- authors:      Thor Kampefner <thorck@pm.me>
- crate_name:   diesel_demo
- crate_type:   bin
- os-arch:      linux-x86_64
- username:     Thor Kampefner
- within_cargo: false
- is_init:      false
- now:          2024-02-20
- bin or lib:   bin 
- advanced:     advanced 
- cli:         
- license:      license 
- ci:           ci 
- itests:      
- benches:     
- async:        async 
- server:       server 

## let's diesel

[Getting Started with Diesel](https://diesel.rs/guides/getting-started.html)
## install
```sh
sudo apt-get install -y libpq-dev postgresql
cargo install diesel_cli --no-default-features --features postgres
```

## setup
set the database URL and setup
```sh
DATABASE_URL="postgres://username:password@localhost/diesel_demo" # in Secrets.toml
diesel setup
```

### option A: use sql for migrations
`diesel migration generate create_posts`
Add some files to the migrations:
```sql
# up:
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
# down:
DROP TABLE posts
```

This will generate a `schema.rs` file.

### option b: use the diesel table macro for migrations
create a `schema.rs` file with content:
```rust
diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

run `diesel migration generate --diff-schema create_posts`

test the migrations:
```sh
diesel migration run # run migrations
diesel migration redo # roll back with down
```
