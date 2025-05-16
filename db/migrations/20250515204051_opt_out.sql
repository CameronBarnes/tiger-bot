-- migrate:up
CREATE TABLE opt_out_users (
	user_id NUMERIC PRIMARY KEY NOT NULL
);

-- migrate:down
DROP TABLE opt_out_users;
