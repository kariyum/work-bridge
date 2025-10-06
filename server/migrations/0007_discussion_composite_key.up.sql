TRUNCATE TABLE discussions, messages RESTART IDENTITY;

ALTER TABLE discussions
    ADD COLUMN proposal_id INT NOT NULL,
    ADD COLUMN task_id INT NOT NULL,

    ADD CONSTRAINT fk_discussion_proposal
        FOREIGN KEY (proposal_id) REFERENCES proposals (id)
        ON UPDATE CASCADE ON DELETE CASCADE,

    ADD CONSTRAINT fk_discussion_task
        FOREIGN KEY (task_id) REFERENCES tasks (id)
        ON UPDATE CASCADE ON DELETE CASCADE,

    ADD CONSTRAINT uq_discussions_composite
        UNIQUE (task_id, proposal_id);