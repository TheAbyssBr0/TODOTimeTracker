use std::process::exit;

mod ongoing_task_db;
mod supertask_db;
mod task_db;
mod title_db;
mod handlers;
mod filters;

#[tokio::main]
async fn main() {
    let url = "postgres://dbuser:pw@localhost:8080/todos";
    let connection_pool = match sqlx::postgres::PgPool::connect(url).await {
        Ok(conn) => conn,
        Err(_) => {
            eprintln!("Error: Cannot connect to database.");
            exit(1);
        }
    };
    
    match sqlx::migrate!("./migrations").run(&connection_pool).await {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Migrations failed.");
            exit(1);
        }
    };

    let api = filters::combined_filters(connection_pool);
    warp::serve(api).run(([0, 0, 0, 0], 8081)).await;
}