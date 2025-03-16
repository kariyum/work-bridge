-- Add migration script here
CREATE TABLE
    IF NOT EXISTS users (
        email VARCHAR(255) PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        role VARCHAR(255) NOT NULL, -- enum [Recruiter, Freelancer]
        hashed_password VARCHAR(255) NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
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
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
        assignee_id VARCHAR(255) NOT NULL,
        budget FLOAT4 NOT NULL,
        status VARCHAR(255) NOT NULL,
        skills TEXT[] NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS discussions (
        id SERIAL PRIMARY KEY,
        user_ids TEXT[] NOT NULL, -- array of user_id
        created_by VARCHAR(255) NOT NULL, -- admin
        title VARCHAR(255), -- can be null if the user did not set a title for the conversation, UI takes care of generating a title to display
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (created_by) REFERENCES users (email)
    );

CREATE TABLE
    IF NOT EXISTS messages (
        id SERIAL PRIMARY KEY,
        from_user_id VARCHAR(255) NOT NULL,
        discussion_id INT NOT NULL,
        content VARCHAR(255) NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (discussion_id) REFERENCES discussions (id)
    ); -- ordered by created_at most recent first

CREATE TYPE notification_type AS ENUM ('proposal', 'message');

CREATE TABLE
    IF NOT EXISTS notifications (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        content JSONB NOT NULL,
        type notification_type NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (email)
    );

CREATE TYPE proposal_status AS ENUM ('pending', 'accepted', 'rejected', 'cancelled');

CREATE TABLE
    IF NOT EXISTS proposals (
        id SERIAL PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        task_id INT NOT NULL,
        status proposal_status NOT NULL, -- ENUM
        budget NUMERIC,
        content TEXT,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (email),
        FOREIGN KEY (task_id) REFERENCES tasks (id)
    );

CREATE TABLE
    IF NOT EXISTS feature_requests (
        id SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        created_by VARCHAR(255) NOT NULL,
        description VARCHAR(255) NOT NULL,
        ups VARCHAR(255)[] NOT NULL DEFAULT '{}',
        downs VARCHAR(255)[] NOT NULL DEFAULT '{}',
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (created_by) REFERENCES users (email)
);

CREATE TABLE
    IF NOT EXISTS comments (
        id SERIAL PRIMARY KEY,
        created_by VARCHAR(255) NOT NULL,
        post_id SERIAL NOT NULL,
        parent_comment_id INT,
        comment VARCHAR(255) NOT NULL,
        ups VARCHAR(255)[] NOT NULL DEFAULT '{}',
        downs VARCHAR(255)[] NOT NULL DEFAULT '{}',
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (post_id) REFERENCES feature_requests (id),
        FOREIGN KEY (created_by) REFERENCES users (email),
        FOREIGN KEY (parent_comment_id) REFERENCES comments (id)
);