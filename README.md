1. cargo install sea-orm-cli
2. sea-orm-cli migrate init
3. 修改migration项目Cargo.toml文件如下
[dependencies.sea-orm-migration]
version = "0.12.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  # "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  # "sqlx-postgres",         # `DATABASE_DRIVER` feature
  "debug-print",
  "runtime-tokio-native-tls",
  "sqlx-sqlite",
  # "sqlx-postgres",
  # "sqlx-mysql",
]

# 生成实体
sea-orm-cli generate entity -o ./src/entity
