use std::fmt;
use uuid::Uuid;
use graphql_client::{GraphQLQuery, QueryBody};

pub mod flac_file;
pub mod mp3_file;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/music/queries/upsert_music.graphql",
    response_derives = "Debug",
)]
pub struct UpsertMusic;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub trait Music {

    fn path(&self) -> &str;
    fn duration(&self) -> i64;
    fn artist(&self) -> &str;
    fn album(&self) -> &str;
    fn title(&self) -> &str;
    fn genre(&self) -> &str;
    fn track(&self) -> i64;
    fn rating(&self) -> f64;
    fn keywords(&self) -> Vec<String>;
    fn links(&self) -> Vec<String>;
    fn create_upsert(&self, user_id: i64) -> upsert_music::Variables {
        upsert_music::Variables {
            title: self.title().to_string(),
            artist: self.artist().to_string(),
            album: self.album().to_string(),
            genre: self.genre().to_string(),
            duration: self.duration(),
            keywords: self.keywords(),
            links: self.links(),
            track: self.track(),
            rating: self.rating(),
            user_id,
        }
    }
    fn create_upsert_query(&self, user_id: i64, operation_name: String) -> QueryBody<upsert_music::Variables> {
        let output_query = upsert_music::QUERY.replacen(&upsert_music::OPERATION_NAME, &operation_name, 1);
        graphql_client::QueryBody {
            variables: self.create_upsert(user_id),
            query: string_to_static_str(output_query),
            operation_name: string_to_static_str(operation_name),
        }
    }

    fn create_upsert_query2(&self, user_id: i64) -> String{
        let uuid = Uuid::new_v4();
        let operation_name = format!("music_{}", uuid.to_simple());
        format!(r#"
{operation_name}: upsertMusic(
    where: {{
        title: {title}
        album: {album}
        artist: {artist}
        userId: {user_id}
    }}
    input: {{
        music: {{
            title: {title}
            album: {album}
            artist: {artist}
            genre: {genre}
            duration: {duration}
            keywords: {keywords}
            number: {number}
            rating: {rating}
            links: {links}
            userId: {user_id}
        }}
    }}
){{
    clientMutationId
}}"#,
            operation_name=operation_name,
            user_id=user_id,
            artist=serde_json::to_string(self.artist()).unwrap(),
            album=serde_json::to_string(self.album()).unwrap(),
            genre=serde_json::to_string(self.genre()).unwrap(),
            title=serde_json::to_string(self.title()).unwrap(),
            number=serde_json::to_string(&self.track()).unwrap(),
            duration=serde_json::to_string(&self.duration()).unwrap(),
            rating=serde_json::to_string(&self.rating()).unwrap(),
            links=serde_json::to_string(&self.links()).unwrap(),
            keywords=serde_json::to_string(&self.keywords()).unwrap(),
        ).replace("\n", "")
    }

    fn create_upsert_query3(&self, user_id: i64, operation_name: &String) -> String{
        format!(r#"
mutation {operation_name} {{
    upsertMusic(
        where: {{
            title: {title}
            album: {album}
            artist: {artist}
            userId: {user_id}
        }}
        input: {{
            music: {{
                title: {title}
                album: {album}
                artist: {artist}
                genre: {genre}
                duration: {duration}
                keywords: {keywords}
                number: {number}
                rating: {rating}
                links: {links}
                userId: {user_id}
            }}
        }}
    ){{
        clientMutationId
    }}
}}"#,
            operation_name=operation_name,
            user_id=user_id,
            artist=serde_json::to_string(self.artist()).unwrap(),
            album=serde_json::to_string(self.album()).unwrap(),
            genre=serde_json::to_string(self.genre()).unwrap(),
            title=serde_json::to_string(self.title()).unwrap(),
            number=serde_json::to_string(&self.track()).unwrap(),
            duration=serde_json::to_string(&self.duration()).unwrap(),
            rating=serde_json::to_string(&self.rating()).unwrap(),
            links=serde_json::to_string(&self.links()).unwrap(),
            keywords=serde_json::to_string(&self.keywords()).unwrap(),
        ).replace("\n", "")
    }
}

impl fmt::Debug for dyn Music {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Music")
         .field("artist", &self.artist())
         .field("album", &self.album())
         .field("title", &self.title())
         .field("track", &self.track())
         .field("duration", &self.duration())
         .field("rating", &self.rating())
         .field("keywords", &self.keywords())
         .field("links", &self.links())
         .finish()
    }
}

impl fmt::Display for dyn Music {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path())
    }
}
