do
$do$
begin
    create extension if not exists "dblink";
    perform dblink_exec(
        'user={admin_user} password={admin_password} host={host} port=5432',
        'drop database if exists {name}'
    );
end
$do$;
