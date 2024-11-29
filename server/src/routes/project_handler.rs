use crate::repository;
use crate::repository::project::{delete_project, insert_project, put_project, ProjectInsert, ProjectRaw};
use crate::repository::tasks::{insert_tasks_sequentially, update_task, CreateTask, RawTask};
use crate::services::token::Claims;
use crate::tasks::repo::TaskCreate;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    tasks: Option<Vec<RawTask>>,
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
            tasks: None,
        }
    }
}

async fn get_projects(_: Claims, pgpool: web::Data<PgPool>) -> impl Responder {
    let projects: Vec<ProjectResponse> = repository::project::get_projects(pgpool.as_ref())
        .await
        .expect("Failed to get projects")
        .into_iter()
        .map(ProjectResponse::from)
        .collect();

    HttpResponse::Ok().json(projects)
}

async fn get_project(
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

async fn get_project_with_tasks(
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

#[derive(Deserialize)]
struct ProjectPost {
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
    tasks: Vec<TaskPost>,
}

#[derive(Deserialize)]
struct TaskPost {
    id: Option<i32>,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    assignee_id: String,
    budget: f32,
    status: String
}

async fn create_project_handler(
    Json(project_post): Json<ProjectPost>,
    pgpool: web::Data<PgPool>,
    claims: Claims,
) -> impl Responder {
    let project_create = ProjectInsert {
        user_id: claims.sub,
        title: project_post.title,
        content: project_post.content,
        deadline: project_post.deadline,
        budget: project_post.budget,
        currency_code: project_post.currency_code,
    };

    let project_raw = insert_project(project_create, pgpool.as_ref())
        .await
        .expect("Failed to insert project");

    let tasks_insert = project_post.tasks.into_iter()
        .map(|task| {
            CreateTask {
                project_id: project_raw.id,
                title: task.title,
                content: task.content,
                deadline: task.deadline,
                assignee_id: task.assignee_id,
                budget: task.budget,
                status: task.status
            }
        })
        .collect::<Vec<CreateTask>>();

    insert_tasks_sequentially(tasks_insert, pgpool.as_ref())
        .await.expect("Failed to insert tasks");

    HttpResponse::Created().finish()
}

async fn delete_project_handler(
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
    _: Claims,
) -> impl Responder {
    let _ = delete_project(path.into_inner(), pgpool.as_ref())
        .await
        .expect("Failed to delete project");

    HttpResponse::Ok().finish()
}

async fn put_project_handler(
    path: Path<i32>,
    Json(project_post): Json<ProjectPost>,
    pgpool: web::Data<PgPool>,
    claims: Claims,
) -> impl Responder {
    let project_id = path.into_inner();
    let project_insert = ProjectInsert {
        user_id: claims.sub,
        title: project_post.title,
        content: project_post.content,
        deadline: project_post.deadline,
        budget: project_post.budget,
        currency_code: project_post.currency_code,
    };
    let response: ProjectResponse = put_project(project_id, project_insert, pgpool.as_ref())
        .await
        .expect(&format!("Failed to put project {}", project_id))
        .into();


    let (tasks_to_update, tasks_to_insert): (Vec<TaskPost>, Vec<TaskPost>) = project_post.tasks.into_iter().partition(|task| {
        task.id.is_some()
    });
    let tasks = tasks_to_insert.into_iter()
        .map(|task| {
            CreateTask {
                project_id: response.id,
                title: task.title,
                content: task.content,
                deadline: task.deadline,
                assignee_id: task.assignee_id,
                budget: task.budget,
                status: task.status
            }
        })
        .collect::<Vec<CreateTask>>();

    insert_tasks_sequentially(tasks, pgpool.as_ref())
        .await.expect("Failed to insert tasks");

    for task in tasks_to_update {
        let pl = CreateTask {
            project_id: response.id,
            title: task.title,
            content: task.content,
            deadline: task.deadline,
            assignee_id: task.assignee_id,
            budget: task.budget,
            status: task.status
        };
        update_task(task.id.unwrap(), pl, pgpool.as_ref()).await.expect("Failed to update task");
    }

    HttpResponse::Ok().json(response)
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("projects")
        .route("", web::get().to(get_projects))
        .route("", web::post().to(create_project_handler))
        .route("/{id}", web::get().to(get_project_with_tasks))
        .route("/{id}", web::delete().to(delete_project_handler))
        .route("/{id}", web::put().to(put_project_handler))
}