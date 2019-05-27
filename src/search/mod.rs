use std::env;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use tantivy::{Index, IndexWriter, TantivyError, Term};
use tantivy::{IndexReader, ReloadPolicy};

mod mission_index;
mod searcher;

use crate::controller::{mission_controller::MissionController, Controller};
use crate::models::missions::Mission;
pub use crate::search::mission_index::*;
use crate::search::searcher::*;
use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    /// Use this instance! use read() when querying and write when adding and updating
    static ref SEARCHER: RwLock<Searcher> = RwLock::new(Searcher::default());
    static ref MISSION_WRITER: Mutex<IndexWriter> = Mutex::new(SEARCHER.read().unwrap().get_writer());
}

pub fn rebuild() {
    info!("Rebuilding index");
    let mut searcher = SEARCHER.write().unwrap();
    searcher.deref_mut().rebuild();
    let new_writer = searcher.deref().get_writer();
    std::mem::replace(MISSION_WRITER.lock().unwrap().deref_mut(), new_writer);
}

pub fn add_mission(new_mission: &Mission) {
    let searcher_guard = SEARCHER.read().unwrap();
    let searcher = searcher_guard.deref();
    let schema = searcher.mission_index.schema();
    let mid = schema.get_field("mid").unwrap();
    let name = schema.get_field("name").unwrap();
    let content = schema.get_field("content").unwrap();
    let mut writer = MISSION_WRITER.lock().unwrap();
    writer.deref_mut().add_document(doc!(
        mid => new_mission.mid as i64,
        name => new_mission.name.clone(),
        content => new_mission.content.clone()
    ));
}

pub fn delete_mission(mission_mid: i32) {
    let searcher_guard = SEARCHER.read().unwrap();
    let searcher = searcher_guard.deref();
    let schema = searcher.mission_index.schema();
    let mid = schema.get_field("mid").unwrap();

    let mid_term = Term::from_field_i64(mid, mission_mid as i64);
    let mut writer = MISSION_WRITER.lock().unwrap();
    writer.deref_mut().delete_term(mid_term);
}

pub fn commit_change() -> Result<(), TantivyError> {
    let mut writer = MISSION_WRITER.lock().unwrap();
    writer.deref_mut().commit()?;
    Ok(())
}
