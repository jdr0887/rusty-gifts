# rusty-gifts

Rusty-gifts is a WebAssembly application using:

- [Seed](https://seed-rs.org/)
- [Actix](https://github.com/actix/actix)
- [Diesel](http://diesel.rs/)

The main intent with this application is to manage gifts between users.  Comes in handy during xmas, or any event, for gifts across a set of users.

Credit goes to Paul Reilly for original implementation.

## Usage

```
$ sudo apt-get install libsqlite3-dev sqlite3
$ echo "DATABASE_URL=gifts.db" > .env
$ diesel migration run
$ cargo run
```

# Started http server: 127.0.0.1:8080