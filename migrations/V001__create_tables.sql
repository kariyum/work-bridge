CREATE TABLE
    IF NOT EXISTS users (
        email VARCHAR(255) PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        role VARCHAR(255) NOT NULL, -- enum [Recruiter, Freelancer]
        hashed_password VARCHAR(255) NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );
CREATE TYPE skill AS ENUM ('developer', 'engineer', 'designer', 'manager', 'analyst', 'consultant', 'other');

CREATE TABLE
    IF NOT EXISTS profiles (
        user_id VARCHAR(255) PRIMARY KEY,
        skills text[], -- array of skill
        birthdate DATE NOT NULL,
        phone VARCHAR(255) NOT NULL,
        -- country VARCHAR(255) NOT NULL,
        -- availability BOOLEAN NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (email)
    );

CREATE TABLE
    IF NOT EXISTS projects (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        deadline TIMESTAMPTZ NOT NULL, -- maybe start_date would be better
        budget FLOAT4 NOT NULL,
        currency_code VARCHAR(255) NOT NULL, -- enum TD
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (email)
    );

CREATE TABLE
    IF NOT EXISTS skills (
        project_id INT NOT NULL, -- could be used to determine needed skills for the whole project
        task_id INT NOT NULL,
        skill VARCHAR(255) NOT NULL, -- enum
        PRIMARY KEY (task_id, skill)
    );

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

CREATE TABLE
    IF NOT EXISTS discussions (
        id SERIAL PRIMARY KEY,
        user_ids TEXT[] NOT NULL, -- array of user_id
        created_by VARCHAR(255) NOT NULL, -- admin
        title VARCHAR(255), -- can be null if the user did not set a title for the converstation, UI takes care of generating a title to display
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (created_by) REFERENCES users (email)
    );

CREATE TABLE
    IF NOT EXISTS messages (
        id SERIAL PRIMARY KEY,
        from_user_id VARCHAR(255) NOT NULL,
        discussion_id INT NOT NULL,
        content VARCHAR(255) NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (discussion_id) REFERENCES discussions (id)
    ); -- ordered by created_at most recent first
 
CREATE TABLE
    IF NOT EXISTS notifications (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        content VARCHAR(255) NOT NULL,
        type VARCHAR(255) NOT NULL, -- enum [message, proposal]
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (email)
    );

CREATE TABLE
    IF NOT EXISTS proposals (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        project_id INT NOT NULL,
        -- status: 0 - pending, 1 - accepted, 2 - rejected, 3 - cancelled
        status INT NOT NULL, -- ENUM
        budget NUMERIC,
        content TEXT,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (email),
        FOREIGN KEY (project_id) REFERENCES projects (id)
    );