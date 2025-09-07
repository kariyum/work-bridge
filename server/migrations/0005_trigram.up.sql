CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX projects_title_trgm_idx ON projects USING GIN (lower(title) gin_trgm_ops);
CREATE INDEX projects_content_trgm_idx ON projects USING GIN (lower(content) gin_trgm_ops);

CREATE INDEX tasks_title_trgm_idx ON tasks USING GIN (lower(title) gin_trgm_ops);
CREATE INDEX tasks_content_trgm_idx ON tasks USING GIN (lower(content) gin_trgm_ops);
