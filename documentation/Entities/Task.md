The task entity lives under a project.
The project owner is the only one that can maintain the tasks.
A task is:
- project id; this task figures under that project id
- title
- content
- deadline
- assignee
- budget
- list of skills


```
CREATE TABLE
    IF NOT EXISTS tasks (
        id SERIAL PRIMARY KEY,
        project_id INT NOT NULL,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        deadline TIMESTAMPTZ NOT NULL,
        assignee VARCHAR(255) NOT NULL,
        bugdet NUMERIC NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (project_id) REFERENCES projects (id)
    );
```

skills are modeled in a separate table which is 
```
CREATE TABLE
    IF NOT EXISTS project_skills (
        project_id INT NOT NULL, -- could be used to determine needed skills for the whole project
        task_id INT NOT NULL,
        skill skill NOT NULL, -- enum
        PRIMARY KEY (task_id, skill)
    );
```