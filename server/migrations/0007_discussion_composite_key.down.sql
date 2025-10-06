ALTER TABLE discussions
    DROP CONSTRAINT uq_discussions_composite,
    DROP CONSTRAINT fk_discussion_proposal,
    DROP CONSTRAINT fk_discussion_task,

    DROP COLUMN proposal_id,
    DROP COLUMN task_id;
