use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
    title: String,
    supertask_id: i32,
    checked: bool
}

pub async fn create_task(pg_pool: &sqlx::PgPool, supertask_id: i32, title: String) ->
    Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO tasks (title, supertask_id, checked) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(title)
        .bind(supertask_id)
        .bind(false)
        .execute(pg_pool)
        .await?;
    Ok(())
}

pub async fn read_supertask_tasks(pg_pool: &sqlx::PgPool, supertask_id: i32) -> Vec<Task> {
    sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE supertask_id = $1",
        supertask_id
    ).fetch_all(pg_pool).await.unwrap_or_default()
}

pub async fn update_task_title(pg_pool: &sqlx::PgPool, id: i32, new_title: String) ->
    Result<(), Box<dyn std::error::Error>> {
    let query = "UPDATE tasks SET title = ($1) WHERE id = ($2)";

    sqlx::query(query)
        .bind(new_title)
        .bind(id)
        .execute(pg_pool)
        .await?;

    Ok(())
}

pub async fn update_toggle_checkbox(pg_pool: &sqlx::PgPool, id: i32) ->
    Result<bool, Box<dyn std::error::Error>> {
    let query = "UPDATE tasks SET checked = NOT checked WHERE id = $1 RETURNING checked";
    let res: bool = sqlx::query_scalar(query)
        .bind(id)
        .fetch_one(pg_pool)
        .await?;

    Ok(res)
}

pub async fn delete_task(pg_pool: &sqlx::PgPool, id: i32) ->
    Result<(), Box<dyn std::error::Error>> {
    let update_query = "UPDATE ongoing SET task_id = $2 WHERE task_id = $1";
    sqlx::query(update_query)
        .bind(id)
        .bind(-1)
        .execute(pg_pool)
        .await?;

    let del_query = "DELETE FROM tasks WHERE id = $1";
    sqlx::query(del_query)
        .bind(id)
        .execute(pg_pool)
        .await?;

    Ok(())
}

pub async fn delete_all_supertask_tasks(pg_pool: &sqlx::PgPool, supertask_id: i32) ->
    Result<(), Box<dyn std::error::Error>> {
    let query = "DELETE FROM tasks WHERE supertask_id = $1";
    sqlx::query(query)
        .bind(supertask_id)
        .execute(pg_pool)
        .await?;

    Ok(())
}