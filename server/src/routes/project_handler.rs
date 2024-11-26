use crate::repository;
use crate::repository::project::ProjectRaw;
use crate::security::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Path;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use crate::repository::tasks::RawTask;

#[derive(Serialize)]
struct ProjectResponse {
    id: i32,
    user_id: String,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
    created_at: DateTime<Utc>,
    tasks: Option<Vec<RawTask>>
}

impl From<ProjectRaw> for ProjectResponse {
    fn from(project: ProjectRaw) -> Self {
        ProjectResponse {
            id: project.id,
            user_id: project.user_id,
            title: project.title,
            content: project.content,
            deadline: project.deadline,
            budget: project.budget,
            currency_code: project.currency_code,
            created_at: project.created_at,
            tasks: None
        }
    }
}

pub async fn get_projects(_: Claims, pgpool: web::Data<PgPool>) -> impl Responder {
    let projects: Vec<ProjectResponse> = repository::project::get_projects(pgpool.as_ref())
        .await
        .expect("Failed to get projects")
        .into_iter()
        .map(ProjectResponse::from)
        .collect();

    HttpResponse::Ok().json(projects)
}

pub async fn get_project(
    _: Claims,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let project_id = path.into_inner();
    let project_option = repository::project::get_project_by_id(project_id, pgpool.as_ref())
        .await
        .expect(&format!("Failed to get project by id {}", project_id))
        .map(ProjectResponse::from);
    if let Some(project) = project_option {
        HttpResponse::Ok().json(project)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn get_project_with_tasks(
    _: Claims,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let project_id = path.into_inner();
    let project_option = repository::project::get_project_by_id(project_id, pgpool.as_ref())
        .await
        .expect(&format!("Failed to get project by id {}", project_id))
        .map(ProjectResponse::from);

    let tasks = repository::tasks::read_tasks_by_project_id(project_id, pgpool.as_ref())
        .await
        .expect(&format!("Failed to read tasks {}", project_id));

    if let Some(project) = project_option {
        let response = ProjectResponse {
            tasks: Some(tasks),
            ..project
        };

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/projects")
        .route("", web::get().to(get_projects))
        .route("/{id}", web::get().to(get_project_with_tasks))
}