do
$do$
begin
    create extension if not exists "dblink";
    if not exists (select from pg_catalog.pg_roles where rolname = '{user}') then
        create role {user} login password '{password}' createdb createrole;
    end if;
end
$do$;
