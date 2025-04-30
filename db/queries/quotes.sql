--! get_name
SELECT
	name
FROM user_names
WHERE id = (:user_id);

--! add_quote (text_content?, file_id?)
INSERT INTO 
	quotes (msg_id, user_from, chat_id, quoted_by, msg_type, msg_date, has_spoiler, text, file_id)
VALUES (:msg_id, :user_from, :chat_id, :quoted_by, :msg_type, :msg_date, :has_spoiler, :text_content, :file_id);

--: Quote(text?, file_id?)

--! random_quote : Quote
SELECT
	user_from,
	chat_id,
	quoted_by,
	msg_type,
	msg_date,
	has_spoiler,
	text,
	file_id
FROM quotes
WHERE chat_id = (:chat_id)
ORDER BY RANDOM()
LIMIT 1;

--! get_quote : Quote
SELECT
	user_from,
	chat_id,
	quoted_by,
	msg_type,
	msg_date,
	has_spoiler,
	text,
	file_id
FROM quotes
WHERE msg_id = (:msg_id);

--! number_of_quotes
SELECT 
	COUNT(*)
FROM quotes
WHERE chat_id = (:chat_id);

--! most_quoted
SELECT
	user_from,
	COUNT(*) AS count
FROM quotes
WHERE chat_id = (:chat_id)
GROUP BY user_from
ORDER BY count DESC
LIMIT 5;

--! most_quoted_by
SELECT
	quoted_by,
	COUNT(*) AS count
FROM quotes
WHERE chat_id = (:chat_id)
GROUP BY quoted_by
ORDER BY count DESC
LIMIT 5;

--! quote_from_user : Quote
SELECT
	user_from,
	chat_id,
	quoted_by,
	msg_type,
	msg_date,
	has_spoiler,
	text,
	file_id
FROM quotes
WHERE chat_id = (:chat_id)
	AND user_from = (:user_from)
ORDER BY RANDOM()
LIMIT 1;

--! search_quote : Quote
SELECT
	user_from,
	chat_id,
	quoted_by,
	msg_type,
	msg_date,
	has_spoiler,
	text,
	file_id
FROM quotes
WHERE chat_id = (:chat_id)
	AND textsearchable_index_col @@ to_tsquery(:query)
ORDER BY RANDOM()
LIMIT 1;

--! search_quote_from_user : Quote
SELECT
	user_from,
	chat_id,
	quoted_by,
	msg_type,
	msg_date,
	has_spoiler,
	text,
	file_id
FROM quotes
WHERE chat_id = (:chat_id)
	AND user_from = (:user_from)
	AND textsearchable_index_col @@ to_tsquery(:query)
ORDER BY RANDOM()
LIMIT 1;
