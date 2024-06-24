use crate::music::filter::Filters;
use crate::music::playlist::PLAYLIST_QUERY;
use crate::music::playlist::{OutputOptions, Playlist, PlaylistOptions};
use crate::music::MUSIC_FIELDS;
use const_format::concatcp;

use crate::music::errors::CriticalErrorKind;

#[derive(clap::Parser)]
#[clap(about = "Generate bests playlists")]
pub struct Bests {
    /// Minimum playlist size
    #[clap(long, default_value_t = 1)]
    min_playlist_size: u64,

    /// Playlist options
    #[clap(flatten)]
    playlist_options: PlaylistOptions,

    /// Output options
    #[clap(flatten)]
    output_options: OutputOptions,

    /// Global filter
    #[clap(flatten)]
    filters: Filters,
}

impl Bests {
    pub async fn bests(
        &self,
        client: edgedb_tokio::Client,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        let mut playlists: Vec<Playlist> = Vec::new();

        for filter in &self.filters.all() {
            let music_filter = serde_json::to_string(filter)?;
            println!("{music_filter}");

            for (name, query) in [
                ("keywords", BESTS_KEYWORDS),
                ("genres", BESTS_GENRES),
                ("ratings", BESTS_RATINGS),
                ("keywords_for_artist", BESTS_KEYWORDS_FOR_ARTIST),
                ("ratings_for_artist", BESTS_RATINGS_FOR_ARTIST),
            ] {
                let now = std::time::Instant::now();
                let bests_genres: Vec<Playlist> = client.query(query, &(&music_filter,)).await?;
                playlists.extend(bests_genres);
                println!("{name}: {:.2?}", now.elapsed());
            }
        }

        for playlist in playlists.iter() {
            if (playlist.len() as u64) < self.min_playlist_size {
                println!("{} : size < {}", playlist.name(), self.min_playlist_size);
                continue;
            }
            let output_options = if let Some(out) = self.output_options.out() {
                OutputOptions::new(
                    self.output_options.output(),
                    &Some(format!("{}/{}.m3u", out, playlist.name())),
                )
            } else {
                self.output_options.clone()
            };
            println!("\nGenerating {} : {}", playlist.name(), playlist.len());
            playlist.generate(&output_options, &self.playlist_options, dry)?;
        }
        Ok(())
    }
}

const BESTS_RATINGS_FOR_ARTIST: &str = concatcp!(
    r#"
    with
    musics := ("#,
    PLAYLIST_QUERY,
    r#")
    for artist_rating_grouping in (group musics by .artist, .rating)
    union (
        select {
            name := artist_rating_grouping.key.artist.name ++ "/rating_" ++ <str>artist_rating_grouping.key.rating,
            musics := artist_rating_grouping.elements {
            "#,
    MUSIC_FIELDS,
    r#" 
            }
        }
    )
    "#
);

const BESTS_KEYWORDS_FOR_ARTIST: &str = concatcp!(
    r#"
    with
    musics := ("#,
    PLAYLIST_QUERY,
    r#")
    for artist in (select distinct musics.artist)
    union (
        with 
            keywords := (
                with
                artist_musics := artist.musics,
                artist_keywords := (select distinct (for music in artist_musics union (music.keywords)))
                for artist_keyword in (select artist_keywords)
                union (
                    select {
                        keyword_name := artist_keyword.name,
                        artist_name := artist.name,
                        musics := (select artist_musics filter artist_keyword in .keywords)
                    }
                )
            )
            for keyword in keywords
            union (
                select {
                    name := keyword.artist_name ++ "/keyword_" ++ std::str_lower(keyword.keyword_name),
                    musics := keyword.musics {
                    "#,
    MUSIC_FIELDS,
    r#"
                    }
                }
            )
    );
"#
);

const BESTS_RATINGS: &str = concatcp!(
    r#"
    with
    musics := ("#,
    PLAYLIST_QUERY,
    r#")
    for rating_grouping in (group musics by .rating)
    union (
        select rating_grouping {
            name := "rating_" ++ <str>.key.rating,
            musics := .elements {
            "#,
    MUSIC_FIELDS,
    r#"
            }
        }
    );
"#
);

const BESTS_GENRES: &str = concatcp!(
    r#"
    with
    musics := ("#,
    PLAYLIST_QUERY,
    r#")
    for genre_grouping in (group musics using genre := .genre.name by genre)
    union (
        select genre_grouping {
            name := "genre_" ++ std::str_lower(.key.genre),
            musics := .elements {
            "#,
    MUSIC_FIELDS,
    r#"
        }}
    );
"#
);

const BESTS_KEYWORDS: &str = concatcp!(
    r#"
    with
    musics := ("#,
    PLAYLIST_QUERY,
    r#"),
    for unique_keyword in (select distinct (for music in musics union (music.keywords)))
        union (
            with musics := (select Keyword filter .name = unique_keyword.name).musics
            select {
                name := "keyword_" ++ std::str_lower(unique_keyword.name),
                musics := (
                    select distinct musics {
                        "#,
    MUSIC_FIELDS,
    r#"
                    }
                    
                )
            }
        )
"#
);
