SELECT * FROM users
WHERE
    email = $1
    AND hashed_password = $2;