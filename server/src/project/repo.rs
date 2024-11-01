use crate::security::token::validate_jwt;
use actix_web::{
    delete, get, post,
    web::{self, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct ProjectRow {
    pub id: i32,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub budget: f32,
    pub currency_code: String,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct ProjectCreate {
    title: String,
    content: String,
    budget: f32,
    currency_code: String,
    deadline: chrono::DateTime<chrono::Utc>,
}

#[post("projects")]
pub async fn create_project(
    request: HttpRequest,
    project_create: Json<ProjectCreate>,
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

    let projects = sqlx::query_as::<_, ProjectRow>(
        "INSERT INTO projects (
        user_id, 
        title, 
        content, 
        budget, 
        currency_code, 
        deadline
    ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&cookie.unwrap().sub)
    .bind(&project_create.title)
    .bind(&project_create.content)
    .bind(&project_create.budget)
    .bind(&project_create.currency_code)
    .bind(&project_create.deadline)
    .fetch_optional(&mut *client)
    .await
    .expect("Failed to insert project into database");

    HttpResponse::Created().json(projects)
}

#[get("projects")]
pub async fn get_projects(request: HttpRequest, pgpool: web::Data<PgPool>) -> impl Responder {
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

    let projects = sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects")
        .fetch_all(&mut *client)
        .await
        .expect("Failed to insert project into database");

    HttpResponse::Ok().json(projects)
}

#[delete("projects/{id}")]
pub async fn delete_project(
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

    let project_id = path.into_inner();
    let _ = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(&project_id)
        .execute(&mut *client)
        .await
        .expect(format!("Failed to delete project from the database {project_id}").as_str());
    
    HttpResponse::Ok().finish()
}

#[get("projects/{id}")]
pub async fn get_project(
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

    let project_id = path.into_inner();
    let project = sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects WHERE id = $1")
        .bind(&project_id)
        .fetch_optional(&mut *client)
        .await
        .expect(format!("Failed to delete project from the database {project_id}").as_str());
    
    HttpResponse::Ok().json(project)
}
