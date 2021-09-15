do
$do$
begin
    create extension if not exists "dblink";
    if not exists (select from pg_catalog.pg_database where datname = '{name}') then
        perform dblink_exec(
            'user={admin_user} password={admin_password} host={host} port=5432',
            'create database {name} with owner {user}'
        );
    end if;
end
$do$;
