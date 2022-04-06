use dioxus::prelude::*;
use im_rc::HashMap;
use include_dir::include_dir;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static DATA_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/data/raw");

// pub static RUN_DATA: Atom<RunDataStore> = |_| {
//     let mut runs = im_rc::HashMap::new();
//     // for file in DATA_DIR.files() {
//     //     let name = file
//     //         .path()
//     //         .file_name()
//     //         .unwrap()
//     //         .to_str()
//     //         .unwrap()
//     //         .trim_end_matches(".json");

//     //     if let Ok(id) = Uuid::parse_str(name) {
//     //         let run = load_run(
//     //             serde_json::from_str::<Vec<UartUpdate>>(file.contents_utf8().unwrap()).unwrap(),
//     //         );

//     //         runs.insert(id, run);
//     //     }
//     // }

//     RunDataStore { runs }
// };
