pub const ARTISTS_QUERY: &str = r#"
select Artist {
    name,
    rating,
    length,
    duration,
    size,
    all_keywords := array_agg(.keywords.name),
    all_genres := array_agg(.musics.genre.name),
    n_albums := count(.albums),
    n_musics := count(.musics)
}
order by .name
"#;
