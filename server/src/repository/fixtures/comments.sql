INSERT INTO users (email, first_name, last_name, role, hashed_password)
VALUES ('user@gmail.com', 'first', 'last', 'role', 'password');

INSERT INTO feature_requests (title, created_by, description, ups, downs)
VALUES ('Feature Title', 'user@gmail.com', 'Description', '{}', '{}');

INSERT INTO comments (post_id, comment, created_by)
VALUES (1, 'comment body', 'user@gmail.com');

INSERT INTO comments (post_id, parent_comment_id, comment, created_by)
VALUES (1, 1, 'comment body', 'user@gmail.com');