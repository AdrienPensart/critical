create schema if not exists musicbot_extensions authorization {user};
grant usage on schema musicbot_extensions to {user};
grant execute on all functions in schema musicbot_extensions to {user};
alter default privileges in schema musicbot_extensions grant execute on functions to {user};
alter default privileges in schema musicbot_extensions grant usage on types to {user};

create extension if not exists "hstore" schema musicbot_extensions;
create extension if not exists "pg_trgm" schema musicbot_extensions;
create extension if not exists "pg_stat_statements" schema musicbot_extensions;
create extension if not exists "pgcrypto" schema musicbot_extensions;
create extension if not exists "pgjwt" schema musicbot_extensions;
