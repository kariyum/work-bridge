ALTER TABLE proposals DROP CONSTRAINT proposals_task_id_fkey,
    ADD CONSTRAINT proposals_task_id_fkey FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE;