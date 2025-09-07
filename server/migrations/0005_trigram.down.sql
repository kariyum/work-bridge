DROP INDEX IF EXISTS projects_title_trgm_idx;
DROP INDEX IF EXISTS projects_content_trgm_idx;

DROP INDEX IF EXISTS tasks_title_trgm_idx;
DROP INDEX IF EXISTS tasks_content_trgm_idx;

DROP EXTENSION IF EXISTS pg_trgm;
