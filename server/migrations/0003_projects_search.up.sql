ALTER TABLE projects
    ADD COLUMN search_vector tsvector;

UPDATE projects
SET search_vector = subquery.new_vector
    FROM (
        WITH all_skills AS (
            SELECT t.project_id as project_id, unnest(t.skills) AS skill
            FROM tasks t
        )
        SELECT
            p.id AS project_id,
            setweight(to_tsvector('english', coalesce(p.title, '')), 'A') ||
            setweight(to_tsvector('english', coalesce(p.content, '')), 'B') ||
            setweight(to_tsvector('english', coalesce(string_agg(DISTINCT t.title, ' '), '')), 'B') ||
            setweight(to_tsvector('english', coalesce(string_agg(DISTINCT t.content, ' '), '')), 'C') ||
            setweight(to_tsvector('english', coalesce(string_agg(DISTINCT s.skill, ' '), '')), 'C')
            AS new_vector
        FROM
            projects p
        LEFT JOIN
            tasks t ON p.id = t.project_id
        LEFT JOIN
            all_skills s ON p.id = s.project_id
        GROUP BY
            p.id
    ) AS subquery
WHERE projects.id = subquery.project_id;

