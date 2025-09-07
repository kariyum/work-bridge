DROP INDEX IF EXISTS projects_search_idx;

DROP TRIGGER IF EXISTS tasks_search_vector_update ON tasks;

DROP TRIGGER IF EXISTS projects_search_vector_update ON projects;

DROP FUNCTION IF EXISTS update_project_search_vector();