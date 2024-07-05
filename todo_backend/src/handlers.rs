use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use warp::http::StatusCode;

use crate::ongoing_task_db::{create_ongoing, read_ongoing_complete, read_ongoing_incomplete,
    update_ongoing_end, update_ongoing_end_now, update_ongoing_start, delete_ongoing};
use crate::supertask_db::{self, create_supertask, read_supertask, update_supertask_title};
use crate::task_db::{self, create_task, update_task_title, update_toggle_checkbox};
use crate::title_db::get_title;

#[derive(Deserialize)]
pub struct SupertaskBuilder {
    title: String
}

#[derive(Deserialize)]
pub struct Updater {
    id: i32,
    new_title: String
}

#[derive(Deserialize)]
pub struct TaskBuilder {
    supertask_id: i32,
    title: String
}

#[derive(Deserialize)]
pub struct DeleteById {
    id: i32
}

#[derive(Deserialize)]
pub struct UpdateById {
    id: i32
}

#[derive(Deserialize)]
pub struct TimeUpdate {
    id: i32,
    time: String
}

#[derive(Serialize)]
pub struct ToggleSuccess {
    answer: Option<bool>
}

#[derive(Serialize)]
pub struct TimeSuccess {
    answer: Option<String>
}

#[derive(Serialize)]
pub struct Title {
    res: String
}

pub async fn post_supertask(st: SupertaskBuilder, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    println!("Trying with {:#?}", st.title);
    let res_create = create_supertask(&db, st.title).await;

    match res_create {
        Ok(_) => {println!("created!"); Ok(StatusCode::CREATED)},
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_supertasks(db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    let res_vec = read_supertask(&db).await;
    Ok(warp::reply::json(&res_vec))
}

pub async fn put_supertask_title(update_info: Updater, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match update_supertask_title(&db, update_info.id, update_info.new_title).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_supertask(id_struct: DeleteById, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match supertask_db::delete_supertask(&db, id_struct.id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn post_task(task_info: TaskBuilder, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match create_task(&db, task_info.supertask_id, task_info.title).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn put_task_title(update_info: Updater, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match update_task_title(&db, update_info.id, update_info.new_title).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn put_task_toggle_checkbox(update_info: UpdateById, db:PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match update_toggle_checkbox(&db, update_info.id).await {
        Ok(b) => Ok(warp::reply::json(
            &ToggleSuccess{
                answer: Some(b)
            })),
        Err(_) => Ok(warp::reply::json(
            &ToggleSuccess{
                answer: None
            }))
    }
}

pub async fn delete_task(id_struct: DeleteById, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match task_db::delete_task(&db, id_struct.id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn post_ongoing(id_struct: UpdateById, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match create_ongoing(&db, id_struct.id).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_ongoing_incomplete(db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    let res_vec = read_ongoing_incomplete(&db).await;
    Ok(warp::reply::json(&res_vec))
}

pub async fn get_ongoing_complete(db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    let res_vec = read_ongoing_complete(&db).await;
    Ok(warp::reply::json(&res_vec))
}

pub async fn put_ongoing_end_now(id_info: UpdateById, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match update_ongoing_end_now(&db, id_info.id).await {
        Ok(s) => Ok(warp::reply::json(&TimeSuccess{answer: Some(s)})),
        Err(_) => Ok(warp::reply::json(&TimeSuccess{answer: None}))
    }
}

pub async fn put_ongoing_start(update_info: TimeUpdate, db: PgPool) ->
    Result<impl warp::Reply, Infallible> {
    match update_ongoing_start(&db, update_info.id, update_info.time).await {
        Ok(s) => Ok(warp::reply::json(&TimeSuccess{answer: Some(s)})),
        Err(_) => Ok(warp::reply::json(&TimeSuccess{answer: None}))
    }
}


pub async fn put_ongoing_end_at(update_info: TimeUpdate, db: PgPool) ->
    Result<impl warp::Reply, Infallible> {
    match update_ongoing_end(&db, update_info.id, update_info.time).await {
        Ok(s) => Ok(warp::reply::json(&TimeSuccess{answer: Some(s)})),
        Err(_) => Ok(warp::reply::json(&TimeSuccess{answer: None}))
    }
}

pub async fn delete_ongoing_handler(delete_info: DeleteById, db: PgPool) -> 
    Result<impl warp::Reply, Infallible> {
    match delete_ongoing(&db, delete_info.id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_title_handler() -> 
    Result <impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&Title{res: get_title()}))
}