use anyhow::{Ok, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    // let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    //     .fetch_all(&pool)
    //     .await?;

    // println!("{:?}", users);

    // insert into database users

    // let user = sqlx::query_as::<_, User>(
    //     "INSERT INTO users (username, email, description) VALUES ($1, $2, $3) RETURNING *",
    // )
    // .bind("John1 Doe")
    // .bind("john1@doe.com")
    // .bind("This is a description")
    // .fetch_one(&pool)
    // .await?;

    // println!("{:?}", user);

    // delete user with id 1
    // sqlx::query("DELETE FROM users WHERE id = $1 RETURNING *")
    //     .bind(1)
    //     .execute(&pool)
    //     .await?;

    // update user with id 2, set username to victor

    sqlx::query("UPDATE users SET username = $1 WHERE id = $2 RETURNING *")
        .bind("victor")
        .bind(3)
        .fetch_one(&pool)
        .await?;

    Ok(())
}
