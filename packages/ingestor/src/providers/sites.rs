use dioxus::prelude::*;
use im_rc::HashMap;
use include_dir::include_dir;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::uart::UartUpdate;
