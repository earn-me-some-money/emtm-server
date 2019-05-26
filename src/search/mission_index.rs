use crate::controller::{mission_controller::MissionController, Controller};
use crate::models::missions::Mission;
use crate::search;
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;

use std::path::Path;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser};
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, TantivyError};

pub trait MissionIndex {
    /// Relatively expensive, please update in bulk
    fn add_mission(&self, new_missions: &[Mission]) -> Result<(), TantivyError>;

    /// Returns a list of mid, ordered by similarities and search score
    fn query_mission(&self, query: &str) -> Result<Vec<(i32, f32)>, TantivyError>;

    /// Init the indexes by loading all missions in database
    /// Mission name and content will be indexed
    fn init_mission_indexes<P: AsRef<Path>>(path: &P) -> Index;

    fn init_mission_data(writer: &mut IndexWriter, schema: &Schema);
}

impl MissionIndex for search::Searcher {
    fn add_mission(&self, new_missions: &[Mission]) -> Result<(), TantivyError> {
        let schema = self.mission_index.schema();
        let mid = schema.get_field("mid").unwrap();
        let name = schema.get_field("name").unwrap();
        let content = schema.get_field("content").unwrap();

        let mut writer = self.mission_index.writer(100_000_000)?;

        for new_mission in new_missions {
            writer.add_document(doc!(
                mid => new_mission.mid as i64,
                name => new_mission.name.clone(),
                content => new_mission.content.clone()
            ));
        }
        writer.commit()?;
        Ok(())
    }

    fn query_mission(&self, query: &str) -> Result<Vec<(i32, f32)>, TantivyError> {
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

    fn init_mission_indexes<P: AsRef<Path>>(path: &P) -> Index {
        let mut builder = SchemaBuilder::default();

        let text_index = TextFieldIndexing::default()
            .set_tokenizer(CANG_JIE)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default().set_indexing_options(text_index);

        builder.add_i64_field("mid", STORED);
        builder.add_text_field("name", text_options.clone());
        builder.add_text_field("content", text_options.clone());

        let schema = builder.build();
        let index = Index::create_in_dir(path, schema.clone()).unwrap();
        index.tokenizers().register(
            CANG_JIE,
            CangJieTokenizer {
                worker: Arc::new(Jieba::empty()),
                option: TokenizerOption::Unicode,
            },
        );
        let mut writer = index.writer(100_000_000).unwrap();

        Self::init_mission_data(&mut writer, &schema);

        index
    }

    fn init_mission_data(writer: &mut IndexWriter, schema: &Schema) {
        let ctrl = Controller::new();
        info!("Loading missions for index initialization");
        let mission_list = ctrl.get_missions_list();
        let mid = schema.get_field("mid").unwrap();
        let name = schema.get_field("name").unwrap();
        let content = schema.get_field("content").unwrap();

        for mission in mission_list {
            writer.add_document(doc!(
                mid => mission.mid as i64,
                name => mission.name,
                content => mission.content
            ));
        }

        writer.commit().unwrap();
    }
}
