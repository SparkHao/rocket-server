
sudo apt install sqlite3

sudo apt install libsqlite3-dev

sudo apt install libpq-dev

sudo apt install libmysqlclient-dev


cargo install diesel_cli --no-default-features --features sqlite

diesel setup --database-url=database.sqlite

diesel migration generate create_products
diesel migration run --database-url=database.sqlite