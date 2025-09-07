CREATE
OR REPLACE FUNCTION update_project_search_vector()
RETURNS TRIGGER AS $$
DECLARE
the_project_id INT;
BEGIN
-- Update the search_vector for the affected project(s)
IF TG_TABLE_NAME = 'projects' THEN
    the_project_id := NEW.id;
ELSIF TG_TABLE_NAME = 'tasks' THEN
    the_project_id := NEW.project_id;
END IF;
UPDATE projects p
SET search_vector = q.search_vector FROM (
    WITH all_skills AS (
        SELECT t.project_id as project_id, unnest(t.skills) AS skill
        FROM tasks t
    )
    SELECT
        p.id AS project_id,
        setweight(to_tsvector('english', coalesce(p.title, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(p.content, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(string_agg(DISTINCT t.title, ' '), '')), 'A') ||
        setweight(to_tsvector('english', coalesce(string_agg(DISTINCT t.content, ' '), '')), 'A') ||
        setweight(to_tsvector('english', coalesce(string_agg(DISTINCT s.skill, ' '), '')), 'B')
        AS search_vector
    FROM projects p
    LEFT JOIN tasks t ON p.id = t.project_id
    LEFT JOIN all_skills s ON p.id = s.project_id
    -- Determine which project to update based on the trigger event
    WHERE p.id = the_project_id
    GROUP BY p.id
) AS q
WHERE p.id = q.project_id;
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- Trigger search_vector update on inserts / updates on the Projects table
CREATE TRIGGER projects_search_vector_update
AFTER UPDATE ON projects
FOR EACH ROW
WHEN (NEW.title IS DISTINCT FROM OLD.title OR NEW.content IS DISTINCT FROM OLD.content)
EXECUTE FUNCTION update_project_search_vector();

CREATE TRIGGER projects_search_vector_update_insert
AFTER INSERT ON projects
FOR EACH ROW
EXECUTE FUNCTION update_project_search_vector();

-- Trigger search_vector update on inserts / updates on the Tasks table
CREATE TRIGGER tasks_search_vector_update
AFTER UPDATE ON tasks
FOR EACH ROW
WHEN (NEW.title IS DISTINCT FROM OLD.title OR NEW.content IS DISTINCT FROM OLD.content)
EXECUTE FUNCTION update_project_search_vector();

CREATE TRIGGER tasks_search_vector_update_insert
AFTER INSERT ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_project_search_vector();

CREATE INDEX projects_search_idx ON projects USING GIN(search_vector);
