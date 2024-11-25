A project is a group of tasks.
Any user can create a project
A project should have:
- A title
- A description
- Deadline
- Budget
- Currency code
- List of Tasks


```
CREATE TABLE
    IF NOT EXISTS projects (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        deadline TIMESTAMPTZ NOT NULL,
        budget FLOAT4 NOT NULL,
        currency_code VARCHAR(255) NOT NULL, -- enum TD
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (email)
    );
```