TRUNCATE discussions CASCADE;
INSERT INTO discussions (user_ids, proposal_id, task_id, created_by)
SELECT ARRAY[p.user_id, pr.user_id] as user_ids, p.id as proposal_id, t.id as task_id, p.user_id
FROM proposals p
    JOIN tasks t ON p.task_id = t.id
    JOIN projects pr ON pr.id = t.project_id
;