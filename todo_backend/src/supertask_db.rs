use serde::{Deserialize, Serialize};

use crate::task_db::{self, delete_all_supertask_tasks, read_supertask_tasks};

/// Returned as JSON from API end point
#[derive(Serialize, Deserialize)]
pub struct Supertask {
    id: i32,
    title: String,
    tasks: Vec<task_db::Task>
}

/// For reading the table
struct TableSupertask {
    id: i32,
    title: String
}

pub async fn create_supertask(pg_pool: &sqlx::PgPool, title: String) ->
    Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO supertasks (title) VALUES ($1)";
    sqlx::query(query)
        .bind(title)
        .execute(pg_pool)
        .await?;
    Ok(())
}

pub async fn read_supertask(pg_pool: &sqlx::PgPool) -> Vec<Supertask> {
    let table_supertasks = sqlx::query_as!(
        TableSupertask,
        "SELECT * FROM supertasks",
    ).fetch_all(pg_pool)
    .await
    .unwrap_or_default();

    // Get iterator of futures containing supertasks
    let supertask_future = table_supertasks
        .into_iter()
        .map(|table_supertask| async move {
            Supertask {
                id: table_supertask.id,
                title: table_supertask.title,
                tasks: read_supertask_tasks(pg_pool, table_supertask.id).await
            }
        }
    );
    
    //... and create a vector from the iterator
    futures::future::join_all(supertask_future).await
}

pub async fn update_supertask_title(pg_pool: &sqlx::PgPool, id: i32, new_title: String) ->
    Result<(), Box<dyn std::error::Error>> {
    let query = "UPDATE supertasks SET title = ($1) WHERE id = ($2)";
    sqlx::query(&query)
        .bind(new_title)
        .bind(id)
        .execute(pg_pool)
        .await?;

    Ok(())
}

pub async fn delete_supertask(pg_pool: &sqlx::PgPool, id:i32) -> 
    Result<(), Box<dyn std::error::Error>> {
    delete_all_supertask_tasks(pg_pool, id).await?;
    let query = "DELETE FROM supertasks WHERE id = $1";
    sqlx::query(&query)
        .bind(id)
        .execute(pg_pool)
        .await?;

    Ok(())
}