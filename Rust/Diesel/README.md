# Diesel

Diesel is a Safe, Extensible ORM and Query Builder for **Rust**.

Add to following to your Cargo.toml
```toml
[dependencies]
diesel = { version = "1.4.8", features = ["postgres"] }
dotenv = "0.15.0"
```

# Adding .env

This program requires the user to setup the enviroment variable **DATABASE_URL** in .env

Run,
```sh
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

Replace,
* username -> with the username register in postgres
* password -> with the responding password
* diesel_demo -> database that exists in postgres

# Diesel CLI
Diesel CLI helps us to setup migrations, Here, we are using *postgres*
```sh
cargo install diesel_cli --no-default-features --features postgres
```

Then, run `diesel setup`  
Which creates migrations directory with `diesel.toml` file for configuration.

# Creating new migration

Use,
```sh
diesel migration generate create_posts
```

Which in turn, creates two file `up.sql` & `down.sql`
* up.sql -> creating stuff
* down.sql -> redo the stuff being created

up.sql
```sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);
```

down.sql
```sql
DROP TABLE posts;
```

We can apply the new migration using,
```sh
diesel migration run
```

and, to redo run,
```sh
diesel migration redo
```

**Result**: this creates a **src/schema.rs** file in src directory which will be used by our code.

# migration in production

When preparing your app for use in production, you may want to run your migrations during the application’s initialization phase. You may also want to include the migration scripts as a part of your code, to avoid having to copy them to your deployment location/image etc.  

The [diesel_migrations](https://docs.rs/crate/diesel_migrations/latest) crate provides the **embed_migrations!** macro, allowing you to embed migration scripts in the final binary. Once your code uses it, you can simply include **embedded_migrations::run(&db_conn)** at the start of your **main** function to run migrations every time the application starts.