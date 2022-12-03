* Install diesel_cli
```
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```
* Setup .env file, run the following to create db and setup diesel migration
```
diesel setup
```
* Create migration for videos table
```
diesel migration generate videos
```
* Update up.sql and down.sql under videos migration, then run
```
diesel migration run
```
* To rollback migration
```
diesel migration revert
```
* Can Redo
```
diesel migration redo
```