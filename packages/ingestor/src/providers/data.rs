use dioxus::prelude::*;
use im_rc::HashMap;
use include_dir::include_dir;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::uart::UartUpdate;

static DATA_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/data/raw");

pub static RUN_DATA: Atom<RunDataStore> = |_| {
    let mut runs = im_rc::HashMap::new();
    for file in DATA_DIR.files() {
        let name = file
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".json");
        let id = Uuid::parse_str(name).unwrap();
        let run = load_run(
            serde_json::from_str::<Vec<UartUpdate>>(file.contents_utf8().unwrap()).unwrap(),
        );

        runs.insert(id, run);
    }

    RunDataStore { runs }
};

pub fn load_run(updates: Vec<UartUpdate>) -> RunVertical {
    let mut gyro_x = vec![];
    let mut gyro_y = vec![];
    let mut gyro_z = vec![];

    let mut accel_x = vec![];
    let mut accel_y = vec![];
    let mut accel_z = vec![];

    for update in updates.iter() {
        match update {
            UartUpdate::Ranging {
                gyro,
                accel,
                distance,
                id,
                temp,
            } => {
                gyro_x.push(gyro.0);
                gyro_y.push(gyro.1);
                gyro_z.push(gyro.2);

                accel_x.push(accel.0);
                accel_y.push(accel.1);
                accel_z.push(accel.2);
            }
        }
    }

    RunVertical {
        updates,
        gyro_x,
        gyro_y,
        gyro_z,
        accel_x,
        accel_y,
        accel_z,
    }
}

#[derive(Clone)]
pub struct RunDataStore {
    // pub data: RunDataScheme,
    pub runs: im_rc::HashMap<Uuid, RunVertical>,
}

impl RunDataStore {
    pub fn insert_run_data(&mut self) {
        //
    }
}

#[derive(Serialize, Deserialize)]
pub struct RunDataScheme {
    pub runs: HashMap<Uuid, RunData>,
    pub events: HashMap<Uuid, EventData>,
    pub profile: ProfileData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RunData {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub throw: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub data: RunVertical,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RunVertical {
    pub updates: Vec<UartUpdate>,

    pub gyro_x: Vec<f32>,
    pub gyro_y: Vec<f32>,
    pub gyro_z: Vec<f32>,

    pub accel_x: Vec<i16>,
    pub accel_y: Vec<i16>,
    pub accel_z: Vec<i16>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventData {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
    pub runs: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileData {
    name: String,
}
