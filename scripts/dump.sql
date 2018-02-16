--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.6
-- Dumped by pg_dump version 9.6.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = public, pg_catalog;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE __diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE __diesel_schema_migrations OWNER TO postgres;

--
-- Name: commit_file; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE commit_file (
    file_id bigint NOT NULL,
    commit_id bigint NOT NULL,
    file_path text NOT NULL
);


ALTER TABLE commit_file OWNER TO postgres;

--
-- Name: commit_file_commit_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE commit_file_commit_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE commit_file_commit_id_seq OWNER TO postgres;

--
-- Name: commit_file_commit_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE commit_file_commit_id_seq OWNED BY commit_file.commit_id;


--
-- Name: commit_file_file_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE commit_file_file_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE commit_file_file_id_seq OWNER TO postgres;

--
-- Name: commit_file_file_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE commit_file_file_id_seq OWNED BY commit_file.file_id;


--
-- Name: file_analysis; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE file_analysis (
    file_id bigint NOT NULL,
    commit_id bigint NOT NULL
);


ALTER TABLE file_analysis OWNER TO postgres;

--
-- Name: git_repository; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE git_repository (
    id bigint NOT NULL,
    url text NOT NULL
);


ALTER TABLE git_repository OWNER TO postgres;

--
-- Name: github_projects_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE github_projects_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE github_projects_id_seq OWNER TO postgres;

--
-- Name: github_projects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE github_projects_id_seq OWNED BY git_repository.id;


--
-- Name: repository_commit; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE repository_commit (
    commit_id bigint NOT NULL,
    repository_id bigint NOT NULL,
    commit_hash character(40) NOT NULL,
    commit_date timestamp without time zone NOT NULL
);


ALTER TABLE repository_commit OWNER TO postgres;

--
-- Name: repository_commit_commit_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE repository_commit_commit_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE repository_commit_commit_id_seq OWNER TO postgres;

--
-- Name: repository_commit_commit_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE repository_commit_commit_id_seq OWNED BY repository_commit.commit_id;


--
-- Name: repository_commit_repository_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE repository_commit_repository_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE repository_commit_repository_id_seq OWNER TO postgres;

--
-- Name: repository_commit_repository_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE repository_commit_repository_id_seq OWNED BY repository_commit.repository_id;


--
-- Name: commit_file file_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY commit_file ALTER COLUMN file_id SET DEFAULT nextval('commit_file_file_id_seq'::regclass);


--
-- Name: commit_file commit_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY commit_file ALTER COLUMN commit_id SET DEFAULT nextval('commit_file_commit_id_seq'::regclass);


--
-- Name: git_repository id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY git_repository ALTER COLUMN id SET DEFAULT nextval('github_projects_id_seq'::regclass);


--
-- Name: repository_commit commit_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY repository_commit ALTER COLUMN commit_id SET DEFAULT nextval('repository_commit_commit_id_seq'::regclass);


--
-- Name: repository_commit repository_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY repository_commit ALTER COLUMN repository_id SET DEFAULT nextval('repository_commit_repository_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY __diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: commit_file commit_file_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY commit_file
    ADD CONSTRAINT commit_file_pkey PRIMARY KEY (file_id, commit_id);


--
-- Name: file_analysis file_analysis_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY file_analysis
    ADD CONSTRAINT file_analysis_pkey PRIMARY KEY (file_id, commit_id);


--
-- Name: git_repository github_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY git_repository
    ADD CONSTRAINT github_projects_pkey PRIMARY KEY (id);


--
-- Name: repository_commit repository_commit_commit_id_repository_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY repository_commit
    ADD CONSTRAINT repository_commit_commit_id_repository_id_key UNIQUE (commit_id, repository_id);


--
-- Name: repository_commit repository_commit_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY repository_commit
    ADD CONSTRAINT repository_commit_pkey PRIMARY KEY (commit_id);


--
-- Name: git_repository unique_url; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY git_repository
    ADD CONSTRAINT unique_url UNIQUE (url);


--
-- Name: commit_file commit_file_commit_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY commit_file
    ADD CONSTRAINT commit_file_commit_id_fkey FOREIGN KEY (commit_id) REFERENCES repository_commit(commit_id);


--
-- Name: file_analysis file_analysis_file_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY file_analysis
    ADD CONSTRAINT file_analysis_file_id_fkey FOREIGN KEY (file_id, commit_id) REFERENCES commit_file(file_id, commit_id);


--
-- Name: repository_commit repository_commit_repository_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY repository_commit
    ADD CONSTRAINT repository_commit_repository_id_fkey FOREIGN KEY (repository_id) REFERENCES git_repository(id);


--
-- PostgreSQL database dump complete
--

