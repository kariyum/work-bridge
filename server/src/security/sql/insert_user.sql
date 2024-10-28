INSERT INTO public.users (email, hashed_password, role, first_name, last_name) 
VALUES ($1, $2, $3, $4, $5);