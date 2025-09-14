WITH RowsToDelete AS (
    select
        id,
        ROW_NUMBER() over (PARTITION BY (user_id, task_id) ORDER BY id) as rn
    from
        proposals
)
DELETE FROM proposals WHERE id IN (SELECT id FROM RowsToDelete where rn > 1);

ALTER TABLE proposals ADD CONSTRAINT unique_user_proposal_constraint UNIQUE (user_id, task_id);