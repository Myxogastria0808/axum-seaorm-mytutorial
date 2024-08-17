# Setup

1. プロジェクトを一つ作る

```sh
cargo init
```

`Cargo.toml`

Database: sqlx-postgres

DatabaeURL: postgres://root:root@localhost:5432

```toml
[dependencies]
sea-orm = { version = "^0.12.0", features = [ "sqlx-postgres", "postgres://postgres:postgres@localhost:5433", "macros" ] }
```

2. マイグレーションファイルの生成

```sh
ea-orm-cli migrate init
```

3. マイグレーションファイルにテーブルの定義を書く

4. マイグレート

```sh
DATABASE_URL="postgres://postgres:postgres@localhost:5433/test_db" sea-orm-cli migrate refresh
```

5. エンティティを生成

```sh
sea-orm-cli generate entity \
    -u postgres://postgres:postgres@localhost:5433/test_db \
    -o entities/src
```

6. entity/Cargo.toml

```toml
[package]
name = "entity"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "entity"
path = "src/mod.rs"

[dependencies.sea-orm]
version = "^0.12.0"
features = [ "macros" ]
```

7. /Cargo.toml

workspace を利用する。

```toml
[package]
name = "axum-seaorm-mytutorial"
version = "0.1.0"
edition = "2021"


[workspace]
members = [".", "entity", "migration"]

[dependencies]
#axum
axum = "0.7.5"
tokio = {version = "1.39.2", features = ["full"] }
#seaorm
entity = { path = "entity" }
migration = { path = "migration" }
sea-orm = { version = "^0.12.0", features = [ "sqlx-postgres", "runtime-tokio-native-tls", "macros" ] }
```

## 参考サイト

https://www.sea-ql.org/sea-orm-tutorial/

https://github.com/trasherr/Blogging-API/blob/master/src/main.rs
