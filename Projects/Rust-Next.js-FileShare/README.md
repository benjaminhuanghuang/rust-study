# Build a Secure File Sharing App with Rust, Axum, Next.js & ShadCN | End-to-End Encryption

https://www.youtube.com/watch?v=t5w2dauFmhM

https://github.com/AarambhDevHub/file-share-rust-backend
https://github.com/AarambhDevHub/file-share-frontend

## Postgresql docker

```sh
docker images
docker pull postgres

docker run --name my_postgres -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -e POSTGRES_DB=file_share -p 5432:5432 -d postgres


```

## Backend

Setup

```sh
cargo new secureshare_backend --bin

cargo install sqlx-cli --no-default-features --features native-tls,postgres

sqlx migrate add tables
sqlx database create
sqlx migrate run
```
