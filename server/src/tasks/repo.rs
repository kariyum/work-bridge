use crate::security::token::validate_jwt;
use actix_web::{get, post, web::{self, Path}, HttpRequest, HttpResponse, Responder};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, PgPool, Postgres};

#[derive(Serialize, sqlx::FromRow)]
struct TaskRow {
    id: i32,
    project_id: i32,
    title: String,
    content: String,
    deadline: chrono::DateTime<chrono::Utc>,
    assignee: String,
    budget: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct TaskCreate {
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub assignee: String,
    pub budget: i32,
}

#[post("/tasks")]
pub async fn create_task(
    request: HttpRequest,
    task_create: Json<TaskCreate>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    println!("Saving task to database {:?}", task_create.content);
    let mut client = pgpool.acquire().await.unwrap();
    sqlx::query(
        "INSERT INTO tasks (
                project_id,
                title,
                content,
                deadline,
                assignee,
                budget
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            ) RETURNING *",
    )
        .bind(&task_create.project_id)
        .bind(&task_create.title)
        .bind(&task_create.content)
        .bind(&task_create.deadline)
        .bind(&task_create.assignee)
        .bind(&task_create.budget)
        .execute(&mut *client)
        .await
        .expect("Failed to insert task into the database");

    HttpResponse::Ok().body("Successfully inserted task")
}

#[get("/projects/{project_id}/tasks")]
pub async fn get_tasks(
    request: HttpRequest,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");
    let project_id: i32 = path.into_inner();
    let messages = sqlx::query_as::<_, TaskRow>(
        "
        SELECT 
            * 
        FROM 
            tasks
        WHERE
            project_id = $1
        ORDER BY created_at ASC                   
        ",
    )
        .bind(&project_id)
        .fetch_all(&mut *client)
        .await
        .expect("Failed to retrieve messages from the database");

    HttpResponse::Ok().json(messages)
}
