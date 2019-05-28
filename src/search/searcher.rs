use std::env;
use std::path::Path;

use tantivy::{Index, IndexWriter};
use tantivy::{IndexReader, ReloadPolicy};

use crate::controller::Controller;
use crate::search::mission_index::*;
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use std::sync::Arc;

static MISSION_DIR: &str = "mission-index";

pub struct Searcher {
    pub index_path_base: String,
    pub mission_index: Index,
    pub mission_index_reader: Option<IndexReader>,
}

impl Searcher {
    pub fn default() -> Self {
        Self::new(&env::var("EMTM_INDEX_DIR").unwrap_or("./emtm-indexes/".to_string()))
    }

    pub fn new(path: &str) -> Self {
        let mission_index_path = Path::new(path).join(MISSION_DIR);

        if !Path::new(path).exists() || !mission_index_path.exists() {
            info!("Index directory not exist, rebuilding indexes");
            if !Path::new(path).exists() {
                std::fs::create_dir(path).unwrap();
            }
            std::fs::create_dir(&mission_index_path).unwrap();
            // init with default db controller
            Self::init_mission_indexes(&mission_index_path, &Controller::new());
        }

        let mission_index = Index::open_in_dir(mission_index_path).unwrap();

        mission_index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()),
                option: TokenizerOption::Unicode,
            },
        );

        let mission_index_reader = Some(mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap());

        Self {
            index_path_base: path.to_string(),
            mission_index,
            mission_index_reader,
        }
    }

    pub fn get_writer(&self) -> IndexWriter {
        self.mission_index.writer(100_000_000).unwrap()
    }

    pub fn rebuild(&mut self, ctrl: &Controller) {
        self.mission_index_reader = None;
        remove_dir_all::remove_dir_all(&self.index_path_base).unwrap();
        std::fs::create_dir(&self.index_path_base).unwrap();
        let mission_index_path = Path::new(&self.index_path_base).join(MISSION_DIR);
        std::fs::create_dir(&mission_index_path).unwrap();
        Searcher::init_mission_indexes(&mission_index_path, ctrl);

        self.mission_index = Index::open_in_dir(mission_index_path).unwrap();

        self.mission_index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()),
                option: TokenizerOption::Unicode,
            },
        );

        self.mission_index_reader = Some(self
            .mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap());
    }
}
