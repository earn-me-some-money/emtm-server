use std::env;
use std::path::{Path, PathBuf};

use tantivy::{Index, IndexWriter};
use tantivy::{IndexReader, ReloadPolicy};

pub mod mission_index;

pub use crate::search::mission_index::*;
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use std::sync::{Arc, Mutex, RwLock};

pub struct Searcher {
    index_path_base: String,
    mission_index: Index,
    mission_index_reader: IndexReader,
    mission_index_writer: Mutex<IndexWriter>,
}

static MISSION_DIR: &str = "mission-index";

lazy_static! {
    /// Use this instance! use read() when querying and write when adding and updating
    pub static ref SEARCHER: RwLock<Searcher> = RwLock::new(Searcher::default());
}

impl Searcher {
    pub fn default() -> Self {
        Self::new(&env::var("EMTM_INDEX_DIR").unwrap_or("./emtm-indexes/".to_string()))
    }

    pub fn new(path: &str) -> Self {
        let mission_index_path = Path::new(path).join(MISSION_DIR);

        if !Path::new(path).exists() {
            info!("Index directory not exist, rebuilding indexes");
            std::fs::create_dir(path).unwrap();
            std::fs::create_dir(&mission_index_path).unwrap();
            Self::init_mission_indexes(&mission_index_path);
        }

        let mission_index = Index::open_in_dir(mission_index_path).unwrap();

        mission_index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()),
                option: TokenizerOption::Unicode,
            },
        );

        let mission_index_reader = mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();
        let mission_index_writer = mission_index.writer(100_000_000).unwrap();

        Self {
            index_path_base: path.to_string(),
            mission_index,
            mission_index_reader,
            mission_index_writer: Mutex::new(mission_index_writer),
        }
    }

    pub fn rebuild(&mut self) {
        info!("Rebuilding index");
        std::fs::remove_dir_all(&self.index_path_base).unwrap();
        std::fs::create_dir(&self.index_path_base).unwrap();
        let mission_index_path = Path::new(&self.index_path_base).join(MISSION_DIR);
        std::fs::create_dir(&mission_index_path).unwrap();
        Self::init_mission_indexes(&mission_index_path);

        self.mission_index = Index::open_in_dir(mission_index_path).unwrap();

        self.mission_index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()),
                option: TokenizerOption::Unicode,
            },
        );

        self.mission_index_reader = self.mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();
        let mission_index_writer = self.mission_index.writer(100_000_000).unwrap();
        self.mission_index_writer = Mutex::new(mission_index_writer);

    }
}
