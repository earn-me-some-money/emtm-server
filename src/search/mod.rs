use std::env;
use std::path::Path;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter};
use tantivy::{IndexReader, ReloadPolicy};

pub mod mission_index;

pub use crate::search::mission_index::*;

pub struct Searcher {
    mission_index: Index,
    mission_index_reader: IndexReader,
}

static MISSION_DIR: &str = "mission-index";

impl Searcher {
    pub fn default() -> Self {
        Self::new(&env::var("EMTM_INDEX_DIR").unwrap_or("./emtm-indexes/".to_string()))
    }

    pub fn new(path: &str) -> Self {
        let mission_index_path = Path::new(path).join(MISSION_DIR);

        if Path::new(path).exists() {
            info!("Index directory not exist, rebuilding indexes");
            std::fs::create_dir(path).unwrap();
            std::fs::create_dir(&mission_index_path).unwrap();
            Self::init_mission_indexes(&mission_index_path);
        }

        let mission_index = Index::open_in_dir(mission_index_path).unwrap();

        let mission_index_reader = mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        Self {
            mission_index,
            mission_index_reader,
        }
    }
}
