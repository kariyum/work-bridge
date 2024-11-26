INSERT INTO users (email, first_name, last_name, role, hashed_password)
VALUES ('user@gmail.com', 'first', 'last', 'role', 'password');

INSERT INTO projects (user_id, title, content, deadline, budget, currency_code)
    VALUES ('user@gmail.com', 'title1', 'content', '2016-03-26 00:00:00+00:00', 10.2, 'TD');