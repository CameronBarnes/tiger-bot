SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: quote_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.quote_type AS ENUM (
    'Text',
    'Document',
    'Photo',
    'Video',
    'Voice'
);


--
-- Name: merge_username(numeric, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.merge_username(key numeric, data text) RETURNS void
    LANGUAGE plpgsql
    AS $$
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
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: opt_out_users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.opt_out_users (
    user_id numeric NOT NULL
);


--
-- Name: quotes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.quotes (
    msg_id integer NOT NULL,
    user_from numeric NOT NULL,
    chat_id bigint NOT NULL,
    quoted_by numeric NOT NULL,
    msg_type public.quote_type NOT NULL,
    msg_date date NOT NULL,
    has_spoiler boolean NOT NULL,
    text text,
    textsearchable_index_col tsvector GENERATED ALWAYS AS (to_tsvector('english'::regconfig, COALESCE(text, ''::text))) STORED,
    file_id text
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(128) NOT NULL
);


--
-- Name: user_names; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_names (
    id numeric NOT NULL,
    name text NOT NULL
);


--
-- Name: opt_out_users opt_out_users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.opt_out_users
    ADD CONSTRAINT opt_out_users_pkey PRIMARY KEY (user_id);


--
-- Name: quotes quotes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quotes
    ADD CONSTRAINT quotes_pkey PRIMARY KEY (msg_id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: user_names user_names_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_names
    ADD CONSTRAINT user_names_pkey PRIMARY KEY (id);


--
-- Name: quote_chat_from_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX quote_chat_from_index ON public.quotes USING btree (chat_id, user_from);


--
-- Name: quote_text_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX quote_text_index ON public.quotes USING gin (textsearchable_index_col);


--
-- Name: user_names_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX user_names_index ON public.user_names USING btree (id);


--
-- Name: quotes quotes_user_from_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quotes
    ADD CONSTRAINT quotes_user_from_fkey FOREIGN KEY (user_from) REFERENCES public.user_names(id);


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20250417034715'),
    ('20250515204051');
