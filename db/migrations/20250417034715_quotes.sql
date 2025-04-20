-- migrate:up
CREATE TABLE user_names (
	id NUMERIC PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);

CREATE INDEX user_names_index ON user_names (id);

CREATE FUNCTION merge_username(key NUMERIC, data TEXT) RETURNS VOID AS
$$
BEGIN
	-- Update key if exists
	UPDATE user_names
	SET name = data
	WHERE id = key;
	IF found THEN
		RETURN;
	END IF;
	-- Not there so we'll insert it
	BEGIN
		INSERT INTO user_names (id, name)
		VALUES (key, data);
	EXCEPTION WHEN unique_violation THEN
		-- Do nothing, if we've hit a concurrency issue here the value should be valid
	END;
END;
$$
LANGUAGE plpgsql;

CREATE TYPE quote_type AS ENUM ('Text', 'Document', 'Photo', 'Video', 'Voice');

CREATE TABLE quotes (
	msg_id INT PRIMARY KEY NOT NULL,
	user_from NUMERIC NOT NULL REFERENCES user_names(id),
	chat_id BIGINT NOT NULL,
	quoted_by NUMERIC NOT NULL,
	msg_type QUOTE_TYPE NOT NULL,
	msg_date DATE NOT NULL,
	has_spoiler BOOLEAN NOT NULL,
	text TEXT,
	textsearchable_index_col TSVECTOR GENERATED ALWAYS AS (to_tsvector('english', coalesce(text, ''))) STORED,
	file_id TEXT
);

CREATE INDEX quote_chat_from_index ON quotes (chat_id, user_from);
CREATE INDEX quote_text_index ON quotes USING GIN (textsearchable_index_col);

-- migrate:down
DROP FUNCTION merge_username;

DROP INDEX quote_text_index;
DROP INDEX quote_chat_from_index;
DROP TABLE quotes;

DROP TYPE quote_type;

DROP INDEX user_names_index;
DROP TABLE user_names;
