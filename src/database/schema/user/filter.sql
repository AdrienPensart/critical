create table if not exists musicbot_public.filter
(
    id           serial primary key,
    user_id      integer not null references musicbot_public.user (id) on delete cascade default musicbot_public.current_musicbot(),
    name         text not null,
    min_duration integer not null default 0,
    max_duration integer not null default +2147483647,
    min_rating   float not null default 0.0 check (min_rating between 0.0 and 5.0),
    max_rating   float not null default 5.0 check (max_rating between 0.0 and 5.0),
    artists      text[] not null Default '{}',
    no_artists   text[] not null Default '{}',
    albums       text[] not null Default '{}',
    no_albums    text[] not null Default '{}',
    titles       text[] not null Default '{}',
    no_titles    text[] not null Default '{}',
    genres       text[] not null Default '{}',
    no_genres    text[] not null Default '{}',
    keywords     text[] not null Default '{}',
    no_keywords  text[] not null Default '{}',
    shuffle      boolean not null default 'false',
    "limit"      integer not null default +2147483647,
    created_at   timestamp with time zone not null default now(),
    updated_at   timestamp with time zone not null default now(),
    constraint unique_filter unique (name, user_id)
);

comment on table musicbot_public.filter is E'@omit delete';

create index if not exists filter_user_idx on musicbot_public.filter (user_id);

alter table if exists musicbot_public.filter enable row level security;

grant select on table musicbot_public.filter to musicbot_anonymous, musicbot_user;
grant insert, update, delete on table musicbot_public.filter to musicbot_user;
grant usage on sequence musicbot_public.filter_id_seq to musicbot_user;

drop policy if exists insert_filter on musicbot_public.filter cascade;
create policy insert_filter on musicbot_public.filter for insert with check (user_id = musicbot_public.current_musicbot());

drop policy if exists select_filter on musicbot_public.filter cascade;
create policy select_filter on musicbot_public.filter for select using (user_id = musicbot_public.current_musicbot());

drop policy if exists update_filter on musicbot_public.filter cascade;
create policy update_filter on musicbot_public.filter for update using (user_id = musicbot_public.current_musicbot());

drop policy if exists delete_filter on musicbot_public.filter cascade;
create policy delete_filter on musicbot_public.filter for delete using (user_id = musicbot_public.current_musicbot());

create or replace function musicbot_public.delete_filter(name text) returns void as $$
    delete from musicbot_public.filter where name = delete_filter.name;
$$ language sql;

create or replace function musicbot_public.delete_all_filter() returns void as $$
    delete from musicbot_public.filter;
$$ language sql;
