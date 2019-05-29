use std::ops::{Deref, DerefMut};

use tantivy::{IndexWriter, TantivyError, Term};

mod mission_index;
mod searcher;

use crate::models::missions::Mission;
pub use crate::search::mission_index::*;
use crate::search::searcher::*;

use crate::controller::Controller;
use std::sync::{Mutex, RwLock};

lazy_static! {
    /// Use this instance! use read() when querying and write when adding and updating
    static ref SEARCHER: RwLock<Searcher> = RwLock::new(Searcher::default());
    static ref MISSION_WRITER: Mutex<Option<IndexWriter>> = Mutex::new(None);
}

pub fn rebuild(ctrl: &Controller) {
    info!("Rebuilding index");
    let mut writer_guard = MISSION_WRITER.lock().unwrap();
    *writer_guard = None;
    let mut searcher = SEARCHER.write().unwrap();
    searcher.deref_mut().rebuild(ctrl);
    let new_writer = searcher.deref().get_writer();
    drop(searcher);
    *writer_guard = Some(new_writer);
}

pub fn add_mission(new_mission: &Mission) {
    info!("Adding mid {} to index", new_mission.mid);
    let searcher_guard = SEARCHER.read().unwrap();
    let searcher = searcher_guard.deref();
    let schema = searcher.mission_index.schema();
    let mid = schema.get_field("mid").unwrap();
    let name = schema.get_field("name").unwrap();
    let content = schema.get_field("content").unwrap();
    let mut writer = MISSION_WRITER.lock().unwrap();
    if let None = writer.deref() {
        *writer = Some(searcher.deref().get_writer());
    }
    writer.deref_mut().as_mut().unwrap().add_document(doc!(
        mid => new_mission.mid as i64,
        name => new_mission.name.clone(),
        content => new_mission.content.clone()
    ));
}

pub fn delete_mission(mission_mid: i32) {
    info!("Deleting mid {} from index", mission_mid);
    let searcher_guard = SEARCHER.read().unwrap();
    let searcher = searcher_guard.deref();
    let schema = searcher.mission_index.schema();
    let mid = schema.get_field("mid").unwrap();

    let mid_term = Term::from_field_i64(mid, mission_mid as i64);
    let mut writer = MISSION_WRITER.lock().unwrap();
    if let None = writer.deref() {
        *writer = Some(searcher.deref().get_writer());
    }
    writer.deref_mut().as_mut().unwrap().delete_term(mid_term);
}

pub fn query_mission(query: &str) -> Result<Vec<(i32, f32)>, TantivyError> {
    SEARCHER.read().unwrap().deref().query_mission(query)
}

pub fn commit_change() -> Result<(), TantivyError> {
    let mut writer = MISSION_WRITER.lock().unwrap();
    if let None = writer.deref() {
        let searcher_guard = SEARCHER.read().unwrap();
        let searcher = searcher_guard.deref();
        *writer = Some(searcher.deref().get_writer());
    }
    writer.deref_mut().as_mut().unwrap().commit()?;
    info!("Commit indexes change");
    Ok(())
}
