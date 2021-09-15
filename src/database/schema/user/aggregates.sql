--drop aggregate if exists musicbot_public.array_cat_agg(anyarray) cascade;
create or replace aggregate musicbot_public.array_cat_agg(anyarray) (
    sfunc=array_cat,
    stype=anyarray,
    initcond='{}'
);
