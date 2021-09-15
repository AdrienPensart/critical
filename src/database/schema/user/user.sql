create table if not exists musicbot_public.user
(
    id               serial primary key,
    first_name       text check (char_length(coalesce(first_name, '')) < 80),
    last_name        text check (char_length(coalesce(last_name, '')) < 80),
    created_at       timestamp with time zone default now(),
    updated_at       timestamp with time zone default now()
);
alter table if exists musicbot_public.user enable row level security;

create or replace function musicbot_public.current_musicbot()
returns integer as
$$
declare
  user_id integer;
begin
    user_id = current_setting('jwt.claims.user_id', true)::integer;
    if user_id is null then
        raise exception 'Invalid user %', user_id;
    end if;
    return user_id;
end;
$$ language plpgsql stable;

create table if not exists musicbot_private.account (
    user_id          integer primary key references musicbot_public.user(id) on delete cascade,
    email            text not null constraint email_unique unique constraint email_format check (email ~* '^.+@.+\..+$'),
    password_hash    text not null
);

do $$
begin
    create role musicbot_anonymous;
    exception when duplicate_object then
    raise notice 'not creating role musicbot_anonymous -- it already exists';
end
$$;

do $$
begin
    create role musicbot_user;
    exception when duplicate_object then
    raise notice 'not creating role musicbot_user -- it already exists';
end
$$;

do $$
begin
    create role musicbot_postgraphile login password 'musicbot_postgraphile_password';
    exception when duplicate_object then
    raise notice 'not creating role musicbot_postgraphile -- it already exists';
end
$$;

grant musicbot_anonymous to musicbot_postgraphile;
grant musicbot_user to musicbot_postgraphile;

drop type if exists musicbot_public.jwt_token cascade;
create type musicbot_public.jwt_token as (
    role text,
    user_id integer,
    exp int
);

create or replace function musicbot_public.authenticate(
  email text,
  password text
)
returns musicbot_public.jwt_token as
$$
declare
  account musicbot_private.account;
begin
    select a.* into strict account
    from musicbot_private.account as a
    where a.email = authenticate.email;
    if account.password_hash = musicbot_extensions.crypt(authenticate.password, account.password_hash) then
        raise notice 'Token Authorization for user % : %', email, ('musicbot_user', account.user_id, extract(epoch from (now() + interval '1 day')))::musicbot_public.jwt_token;
        return ('musicbot_user', account.user_id, extract(epoch from (now() + interval '1 day')))::musicbot_public.jwt_token;
    else
        raise exception 'Authentication failed for user %', email;
    end if;
exception
    when NO_DATA_FOUND then
        raise exception 'Account % not found', email;
    when TOO_MANY_ROWS then
        raise exception 'Account % not unique', email;
    return null;
end;
$$ language plpgsql strict security definer;

create or replace function musicbot_public.register_user(
    email text,
    password text,
    first_name text default null,
    last_name text default null
)
returns musicbot_public.jwt_token as
$$
declare
    minimum_password_length constant integer := 8;
    cn text;
begin
    if register_user.email is null then
        raise exception 'Email is mandatory';
    end if;
    if register_user.password is null then
        raise exception 'Password is mandatory';
    end if;

    if length(register_user.password) < minimum_password_length then
        raise exception 'Password is too weak, it must be at least % characters long', minimum_password_length;
    end if;

    with insert_user as (
        insert into musicbot_public.user as u (first_name, last_name)
        values (register_user.first_name, register_user.last_name)
        returning *
    )
    insert into musicbot_private.account (user_id, email, password_hash)
    values ((select insert_user.id from insert_user), register_user.email, musicbot_extensions.crypt(register_user.password, musicbot_extensions.gen_salt('bf')));

    return musicbot_public.authenticate(email => register_user.email, password => register_user.password);
exception
    when integrity_constraint_violation or unique_violation then
        get stacked diagnostics cn := constraint_name;
        if cn = 'email_format' then
            raise exception 'Email format is not correct';
        end if;

        if cn = 'email_unique' then
            raise exception 'Email already in use';
        end if;
        raise;
end;
$$ language plpgsql security definer;

create or replace function musicbot_public.unregister_user()
returns musicbot_public.user as
$$
    delete from musicbot_public.user u
    where u.id = musicbot_public.current_musicbot()
    returning *
$$ language sql strict security definer;

alter default privileges revoke execute on functions from public;

grant usage on schema musicbot_public to musicbot_anonymous, musicbot_user;
grant select on table musicbot_public.user to musicbot_anonymous, musicbot_user;
grant update, delete on table musicbot_public.user to musicbot_user;

grant execute on function musicbot_public.authenticate to musicbot_anonymous;
grant execute on function musicbot_public.current_musicbot to musicbot_anonymous;
grant execute on function musicbot_public.register_user to musicbot_anonymous;

drop policy if exists select_user on musicbot_public.user cascade;
create policy select_user on musicbot_public.user for select to musicbot_user using (id = musicbot_public.current_musicbot());

drop policy if exists update_user on musicbot_public.user cascade;
create policy update_user on musicbot_public.user for update to musicbot_user using (id = musicbot_public.current_musicbot());

drop policy if exists delete_user on musicbot_public.user cascade;
create policy delete_user on musicbot_public.user for delete to musicbot_user using (id = musicbot_public.current_musicbot());