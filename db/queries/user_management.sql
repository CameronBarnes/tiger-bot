--! add_opt_out_user
INSERT INTO
	opt_out_users (user_id)
VALUES (:user_id);

--! remove_opt_out_user
DELETE FROM opt_out_users
WHERE user_id = (:user_id);

--! is_user_opt_out
SELECT 1
FROM opt_out_users
WHERE user_id = (:user_id)
LIMIT 1;

--! get_name
SELECT
	name
FROM user_names
WHERE id = (:user_id);

--! remove_name
DELETE FROM user_names
WHERE id = (:user_id);
