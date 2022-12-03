* Install diesel_cli
```
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

* Setup .env file, run the following to create db and setup diesel migration
```
diesel setup
```

