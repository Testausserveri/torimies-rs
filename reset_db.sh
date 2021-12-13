rm -rf database.sqlite*
sqlx database setup --database-url sqlite:database.sqlite || sqlx database setup
