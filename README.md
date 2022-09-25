# Nitomoe

Modern imageboard software

### Deploying

todo: write docker instructions

### Getting started with local development

#### Install Rust and Cargo

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add `~/.cargo/bin` to your PATH

#### Install diesel_cli (required for managing the local DB)

`diesel_cli` requires Postgres, MySQL and SQLite3.

##### macOS
```sh
$ brew install postgresql
$ brew install sqlite3
$ brew install mysql
```

##### Ubuntu / Windows (WSL)
```sh
$ sudo apt install libsql3-dev
$ sudo apt install libmysqlclient-dev
$ sudo apt install libpq-dev
```

##### Install diesel_cli

```sh
$ cargo install diesel_cli
```

#### Setup the environment

```sh
$ cp .env.example .env
```

Edit the `.env` file and supply the expected values

| Key                        | Example Value                                                             |
| -------------------------- | ------------------------------------------------------------------------- |
| DATABASE_URL               | postgres://postgres:postgres@localhost/nitomoe_dev                        |

##### Bring up the development database

This requires the use of Docker. Alternatively, running a Postgres server outside of Docker on your system would work as well, but Docker greatly simplifies the process. If you are using an older version of Docker, replace `docker compose` with `docker-compose`.

```sh
$ docker compose -f dev/docker-compose.yml up -d
$ diesel setup
$ diesel migration run
```