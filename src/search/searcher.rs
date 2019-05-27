use std::env;
use std::path::Path;

use tantivy::{Index, IndexWriter, TantivyError};
use tantivy::{IndexReader, ReloadPolicy};

use crate::controller::Controller;
use crate::search::mission_index::*;
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

static MISSION_DIR: &str = "mission-index";

pub struct Searcher {
    pub index_path_base: String,
    pub mission_index: Index,
    pub mission_index_reader: IndexReader,
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

        let mission_index_reader = mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        Self {
            index_path_base: path.to_string(),
            mission_index,
            mission_index_reader,
        }
    }

    pub fn get_writer(&self) -> IndexWriter {
        self.mission_index.writer(100_000_000).unwrap()
    }

    pub fn query_mission(&self, query: &str) -> Result<Vec<(i32, f32)>, TantivyError> {
        let searcher = self.mission_index_reader.searcher();
        let schema = self.mission_index.schema();
        let mid = schema.get_field("mid").unwrap();
        let name = schema.get_field("name").unwrap();
        let content = schema.get_field("content").unwrap();
        let query_parser = QueryParser::for_index(&self.mission_index, vec![name, content]);
        let parsed_query = query_parser.parse_query(query)?;
        let docs = searcher.search(&parsed_query, &TopDocs::with_limit(100))?;
        let mut results = vec![];

        for (score, doc_address) in docs {
            let doc = searcher.doc(doc_address)?;
            let values = doc.field_values();
            for v in values {
                if v.field() == mid {
                    results.push((v.value().i64_value() as i32, score));
                }
            }
        }

        Ok(results)
    }

    pub fn rebuild(&mut self, ctrl: &Controller) {
        std::fs::remove_dir_all(&self.index_path_base).unwrap();
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

        self.mission_index_reader = self
            .mission_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();
    }
}
