use sqlx::PgPool;
use warp::Filter;

use crate::handlers;

/// Clones the db connection to transfer ownership to handlers when called
fn get_db(db: PgPool) -> 
    impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// API endpoints as a warp filter (check warp docs)
pub fn combined_filters(db: PgPool) -> 
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    post_supertask_filter(db.clone())
    .or(get_supertask_filter(db.clone()))
    .or(put_supertask_filter(db.clone()))
    .or(delete_supertask_filter(db.clone()))
    .or(post_task_filter(db.clone()))
    .or(put_task_title_filter(db.clone()))
    .or(put_task_checkbox_filter(db.clone()))
    .or(delete_task_filter(db.clone()))
    .or(post_ongoing_filter(db.clone()))
    .or(get_ongoing_incomplete_filter(db.clone()))
    .or(get_ongoing_complete_filter(db.clone()))
    .or(put_ongoing_end_now_filter(db.clone()))
    .or(put_ongoing_start_filter(db.clone()))
    .or(put_ongoing_end_at_filter(db.clone()))
    .or(delete_ongoing(db.clone()))
    .or(get_title_filter())
}

fn post_supertask_filter(db: PgPool) -> 
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("supertask")
        .and(warp::post())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::post_supertask)
}

fn get_supertask_filter(db: PgPool) -> 
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("supertask")
        .and(warp::get())
        .and(get_db(db))
        .and_then(handlers::get_supertasks)
}

fn put_supertask_filter(db: PgPool) ->
impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("supertask")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_supertask_title)
}

fn delete_supertask_filter(db: PgPool) -> 
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("supertask")
        .and(warp::delete())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::delete_supertask)
}

fn post_task_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("task")
        .and(warp::post())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::post_task)
}

fn put_task_title_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("task")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_task_title)
}

fn put_task_checkbox_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("task" / "checkbox")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_task_toggle_checkbox)
}

fn delete_task_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("task")
        .and(warp::delete())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::delete_task)
}

fn post_ongoing_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing")
        .and(warp::post())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::post_ongoing)
}

fn get_ongoing_incomplete_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing" / "incomplete")
        .and(warp::get())
        .and(get_db(db))
        .and_then(handlers::get_ongoing_incomplete)
}

fn get_ongoing_complete_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing" / "complete")
        .and(warp::get())
        .and(get_db(db))
        .and_then(handlers::get_ongoing_complete)
}

fn put_ongoing_end_now_filter(db: PgPool) -> 
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing" / "end" / "now")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_ongoing_end_now)
}

fn put_ongoing_start_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing" / "start")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_ongoing_start)
}

fn put_ongoing_end_at_filter(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing" / "end")
        .and(warp::put())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::put_ongoing_end_at)
}

fn delete_ongoing(db: PgPool) ->
    impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ongoing")
        .and(warp::delete())
        .and(warp::body::json())
        .and(get_db(db))
        .and_then(handlers::delete_ongoing_handler)
    
}

fn get_title_filter() ->
impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("weektitle")
        .and(warp::get())
        .and_then(handlers::get_title_handler)
}