use indradb::{Database, MemoryDatastore};

use super::{
    albums::AlbumVertex, artists::ArtistVertex, errors::CriticalErrorKind, folders::FolderVertex,
    genres::GenreVertex, keywords::KeywordVertex, ratings::Rating, MusicVertex,
};

pub struct MusicInput {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub length: i64,
    pub size: u64,
    pub track: i64,
    pub rating: Rating,
    pub keywords: Vec<String>,
    pub folder: String,
    pub path: String,
    pub username: String,
    pub ipv4: String,
}

impl MusicInput {
    pub fn upsert(&self, db: &Database<MemoryDatastore>) -> Result<uuid::Uuid, CriticalErrorKind> {
        let folder_vertex = FolderVertex {
            name: self.folder.to_string(),
            username: self.username.clone(),
            ipv4: self.ipv4.clone(),
        };
        let _folder_vertex_id = folder_vertex.upsert(db)?;
        // println!("folder_vertex_id: {folder_vertex_id}");

        let artist_vertex = ArtistVertex {
            name: self.artist.clone(),
        };
        let artist_vertex_id = artist_vertex.upsert(db)?;
        // println!("artist_vertex_id: {artist_vertex_id}");

        let album_vertex = AlbumVertex {
            name: self.album.clone(),
            artist_id: artist_vertex_id,
        };
        let album_vertex_id = album_vertex.upsert(db)?;

        let genre_vertex = GenreVertex {
            name: self.genre.clone(),
        };
        let genre_vertex_id = genre_vertex.upsert(db)?;
        // println!("genre_vertex_id: {genre_vertex_id}");

        let mut keyword_vertex_ids = Vec::new();
        // {
        for keyword in &self.keywords {
            let keyword_vertex = KeywordVertex {
                name: keyword.clone(),
            };
            let keyword_vertex_id = keyword_vertex.upsert(db)?;
            // println!("keyword_vertex_id: {keyword_vertex_id}");
            keyword_vertex_ids.push(keyword_vertex_id);
        }

        let music_vertex = MusicVertex {
            title: self.title.to_string(),
            artist_id: artist_vertex_id,
            album_id: album_vertex_id,
            genre_id: genre_vertex_id,
            length: self.length,
            track: self.track,
            rating: self.rating,
            keywords: keyword_vertex_ids,
        };
        music_vertex.upsert(db)
    }
}
