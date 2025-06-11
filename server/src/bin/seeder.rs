use fake::{
    Fake, Faker,
};
use server::repository::user::{insert_user, RegisterRequest};

use dotenv::dotenv;
use server::routes::project_handler::seed_projects_handler;
use server::services::database::get_db_pool;
use server::services::token::Claims;
use sqlx::{Pool, Postgres};

async fn seed_users(conn: &Pool<Postgres>) -> Result<RegisterRequest, sqlx::Error> {
    let register_request: RegisterRequest = Faker.fake();
    println!("Inserting {:?}", register_request);
    insert_user(&register_request, conn)
        .await
        .map(|_| register_request)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = get_db_pool().await.expect("Failed to get PG pool");
    println!("Seeding users!");
    let user = seed_users(&pool).await.expect("Failed to insert user");
    // let config = ProjectInsertConfig {
    //     user_ids: vec![user.email],
    // };
    let claims = Claims {
        sub: user.email,
        role: user.role,
        exp: 1
    };
    seed_projects_handler(claims, pool).await;
    // let projects = seed_projects(config, &pool)
    //     .await
    //     .expect("Failed to insert project");
    // let task_config = CreateTaskConfig {
    //     project_ids: vec![projects.id],
    // };
    // seed_tasks(task_config, &pool)
    //     .await
    //     .expect("Failed to insert task");
    Ok(())
}
