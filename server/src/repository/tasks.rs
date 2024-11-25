
struct RawTask {

}

pub async fn get_tasks_by_project_id(project_id: i32) -> Result<Vec<RawTask>, sqlx::Error> {
    Ok(vec![])
}