CREATE TABLE users (
    id bigserial PRIMARY KEY,
    username character varying(64) UNIQUE NOT NULL,
    email character varying(320) UNIQUE NOT NULL,
    first_name character varying(255),
    last_name character varying(255),
    encrypted_password bytea,
    salt bytea,
    creation_date_utc timestamp DEFAULT (now() AT TIME ZONE 'utc')
);