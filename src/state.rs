use std::collections::HashMap;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::project::Project;

pub const ADMIN: Item<Addr> = Item::new("admins");

pub const PROJECTS: Item<HashMap<String, Project>> = Item::new("projects");

pub const PROJECT_OWNERS: Item<Vec<Addr>> = Item::new("project_owners");
