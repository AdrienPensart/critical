use const_format::concatcp;

pub const UPSERT_FOLDER: &str = r#"
select (
    insert Folder {
        name := <str>$0,
        username := <str>$1,
        ipv4 := <str>$2
    }
    unless conflict on (.name, .username, .ipv4) else (select Folder)
).id
"#;

pub const UPSERT_ARTIST: &str = r#"
select (
    insert Artist {
        name := <str>$0
    }
    unless conflict on (.name) else (select Artist)
).id
"#;

pub const UPSERT_ALBUM: &str = r#"
select (
    insert Album {
        name := <str>$0,
        artist := <Artist>$1
    }
    unless conflict on (.name, .artist) else (select Album)
).id
"#;

pub const UPSERT_GENRE: &str = r#"
select (
    insert Genre {
        name := <str>$0
    }
    unless conflict on (.name) else (select Genre)
).id
"#;

pub const UPSERT_KEYWORD: &str = r#"
select (
    insert Keyword {
        name := <str>$0
    }
    unless conflict on (.name)
    else (select Keyword)
).id
"#;

pub const UPSERT_MUSIC: &str = r#"
select (
    insert Music {
        name := <str>$0,
        album := <Album><uuid>$1,
        genre := <Genre><uuid>$2,
        size := <Size>$3,
        length := <Length>$4,
        keywords := (select distinct array_unpack(<array<Keyword>><array<uuid>>$5)),
        track := <Track>$6,
        rating := <Rating>$7,
        folders := (
            (<Folder><uuid>$8) {
                @path := <str>$9
            }
        )
    }
    unless conflict on (.name, .album)
    else (
        update Music
        set {
            genre := <Genre><uuid>$2,
            size := <Size>$3,
            length := <Length>$4,
            keywords := (select distinct array_unpack(<array<Keyword>><array<uuid>>$5)),
            track := <Track>$6,
            rating := <Rating>$7,
            folders += (
                (<Folder><uuid>$8) {
                    @path := <str>$9
                }
            )
        }
    )
).id
"#;

pub const MUSIC_FIELDS: &str = r#"
name,
artist_name := .artist.name,
album_name := .album.name,
genre_name := .genre.name,
length,
human_duration,
size,
human_size,
track,
rating,
keywords_names := (select .keywords.name),
folders: {
    name,
    username,
    ipv4,
    path := @path
}
"#;

pub const PLAYLIST_QUERY: &str = concatcp!(
    r#"
    with music_filter := to_json(<str>$0),
    select Music {
        "#,
    MUSIC_FIELDS,
    r#"
    }
    filter
        .length >= <Length>music_filter['min_length'] and .length <= <Length>music_filter['max_length']
        and .size >= <Size>music_filter['min_size'] and .size <= <Size>music_filter['max_size']
        and .rating >= <Rating>music_filter['min_rating'] and .rating <= <Rating>music_filter['max_rating']
        and re_test(<str>music_filter['artist'], .artist.name)
        and re_test(<str>music_filter['album'], .album.name)
        and re_test(<str>music_filter['genre'], .genre.name)
        and re_test(<str>music_filter['title'], .name)
        and re_test(<str>music_filter['keyword'], array_join(array_agg((select .keywords.name)), " "))
        and (<str>music_filter['pattern'] = "" or ext::pg_trgm::word_similar(<str>music_filter['pattern'], .title))
    order by
        .artist.name then
        .album.name then
        .track then
        .name
    limit <`Limit`>music_filter['limit']
"#
);

pub const REMOVE_PATH_QUERY: &str = r#"
update Music
filter contains(.paths, <str>$0)
set {folders := (select .folders filter @path != <str>$0)};
"#;

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

pub const BESTS_QUERY: &str = concatcp!(
    r#"
with
    musics := ("#,
    PLAYLIST_QUERY,
    r#"),
    unique_keywords := (select distinct (for music in musics union (music.keywords)))
select {
    genres := (
        group musics {
            "#,
    MUSIC_FIELDS,
    r#"
        }
        by .genre
    ),
    keywords := (
        for unique_keyword in unique_keywords
        union (
            select Keyword {
                name,
                musics := (
                    select musics {
                        "#,
    MUSIC_FIELDS,
    r#"
                    }
                    filter unique_keyword.name in .keywords.name
                )
            }
            filter .name = unique_keyword.name
        )
    ),
    ratings := (
        group musics {
            "#,
    MUSIC_FIELDS,
    r#"
        }
        by .rating
    ),
    keywords_for_artist := (
        for artist in (select distinct musics.artist)
        union (
            select {
                artist := artist.name,
                keywords := (
                    with
                    artist_musics := (select musics filter .artist = artist),
                    artist_keywords := (select distinct (for music in artist_musics union (music.keywords)))
                    for artist_keyword in (select artist_keywords)
                    union (
                        select {
                            keyword := artist_keyword.name,
                            musics := (
                                select artist_musics {
                                    "#,
    MUSIC_FIELDS,
    r#"
                                }
                                filter artist_keyword in .keywords
                            )
                        }
                    )
                )
            }
        )
    ),
    ratings_for_artist := (
        group musics {
            "#,
    MUSIC_FIELDS,
    r#"
        }
        by .artist, .rating
    )
}
"#
);
