use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ongoing {
    id: i32,
    task_id: i32,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>
}

/// Gets a UTC date time for today at military time s
/// # Example:
/// `date_from_str("1315");` 
/// returns a UTC date time for today at 1:15PM
fn datetime_from_str(s: &str) -> 
    Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    let time = NaiveTime::parse_from_str(s, "%H%M")?;
    let date = Local::now().date_naive();
    let naive_dt = NaiveDateTime::new(date, time);

    let local = Local::now()
        .timezone()
        .from_local_datetime(&naive_dt)
        .single()
        .unwrap();

    Ok(DateTime::from(local))
}

/// Creates an ongoing item in the database with `start_time` when this function is called
pub async fn create_ongoing(pg_pool: &sqlx::PgPool, task_id: i32) -> 
    Result<String, Box<dyn std::error::Error>> {
    let time_now = Utc::now();
    let query = "INSERT INTO ongoing (task_id, start_time) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(task_id)
        .bind(time_now)
        .execute(pg_pool)
        .await?;

    let local: DateTime<Local> = DateTime::from(time_now);
    Ok(local.format("%H%M").to_string())
}

pub async fn read_ongoing_incomplete(pg_pool: &sqlx::PgPool) -> 
    Vec<Ongoing> {
    sqlx::query_as!(
        Ongoing,
        "SELECT * FROM ongoing WHERE end_time IS NULL",
    ).fetch_all(pg_pool).await.unwrap_or_default()
}

pub async fn read_ongoing_complete(pg_pool: &sqlx::PgPool) -> Vec<Ongoing> {
    sqlx::query_as!(
        Ongoing,
        "SELECT * FROM ongoing WHERE end_time IS NOT NULL ORDER BY end_time DESC LIMIT 10"
    ).fetch_all(pg_pool).await.unwrap_or_default()
}

/// Ends ongoing item in database when this function is called
pub async fn update_ongoing_end_now(pg_pool: &sqlx::PgPool, id: i32) -> 
    Result<String, Box<dyn std::error::Error>> {
    let time_now = Utc::now();
    let query = "UPDATE ongoing SET end_time = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(time_now)
        .bind(id)
        .execute(pg_pool)
        .await?;

    let local: DateTime<Local> = DateTime::from(time_now);
    Ok(local.format("%H%M").to_string())
}

pub async fn update_ongoing_start(pg_pool: &sqlx::PgPool, id: i32, start: String) ->
    Result<String, Box<dyn std::error::Error>> {
    let start_time = datetime_from_str(&start)?;
    let query = "UPDATE ongoing SET start_time = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(start_time)
        .bind(id)
        .execute(pg_pool)
        .await?;

    let local: DateTime<Local> = DateTime::from(start_time);
    Ok(local.format("%H%M").to_string())
}

pub async fn update_ongoing_end(pg_pool: &sqlx::PgPool, id: i32, end: String) -> 
    Result<String, Box<dyn std::error::Error>> {
    let end_time = datetime_from_str(&end)?;
    let query = "UPDATE ongoing SET end_time = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(end_time)
        .bind(id)
        .execute(pg_pool)
        .await?;

        let local: DateTime<Local> = DateTime::from(end_time);
        Ok(local.format("%H%M").to_string())
}

pub async fn delete_ongoing(pg_pool: &sqlx::PgPool, id: i32) -> 
    Result<(), Box<dyn std::error::Error>> {
    let query = "DELETE FROM ongoing WHERE id = $1";

    sqlx::query(query)
        .bind(id)
        .execute(pg_pool)
        .await?;

    Ok(())
}