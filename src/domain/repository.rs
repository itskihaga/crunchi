use crate::repository::{push_record, RecordPushIO, sync_record_update};
use serde_json::{self};
use serde::{Deserialize};

use exprocess::client::{Record, RecordSync, Repository};
use super::state::{AppCommand, AppCore, AppResult};

pub struct AppRepository {
    room_id: String,
    unsync_fn: Option<Box<dyn FnOnce()>>
}

impl AppRepository {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
            unsync_fn: None
        }
    }
}

#[derive(Deserialize)]
pub struct RecordDesirailizeIO {
    id: String,
    command: AppCommand,
    result: AppResult
}

impl Repository<AppCore> for AppRepository {
  
    fn push(&mut self,record: Record<AppCore>) {
        push_record(
            self.room_id.as_str(),
            RecordPushIO {
                id: record.id.as_str(),
                result: serde_json::to_string(&record.result).expect("JSON Serialize Err").as_str(),
                command: serde_json::to_string(&record.command).expect("JSON Serialize Er").as_str()
            }
        )
    }

    fn sync(&mut self,mut listener: Box<dyn FnMut(Vec<RecordSync<AppCore>>)>) {
        self.unsync_fn = Some(sync_record_update(self.room_id.as_str(), move |json| {
            let records : Vec<RecordDesirailizeIO> = serde_json::from_str(&json).expect("JSON Parse Err");
            listener(
                records.iter()
                .map(|record| RecordSync {id: record.id.as_str(), result: &record.result, command: &record.command}) 
                .collect()
            );
        }));
    }

    fn unsync(&mut self) {
        self.unsync_fn.take().map(|call| call());
    }
}

