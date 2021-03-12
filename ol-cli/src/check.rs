//! `check` module

use cli::libra_client::LibraClient;
use reqwest::Url;
// use anyhow::Error;
// use anyhow::{Result};
use libra_json_rpc_client::views::MetadataView;
use sled::{IVec, Db};
use crate::{client::*, management, metadata::Metadata};
use std::path::PathBuf;
use std::str::FromStr;
use sysinfo::{SystemExt, ProcessExt};

const HOME: &str = "~/.0L";
const CHECK_DB: &str = "ol-system-checks";
const SYNC_KEY: &str = "is_synced";

/// Checks we want to make on the node
pub struct Check {
    tree: Db,
    miner_process_name: &'static str,
    node_process_name: &'static str,
}

impl Check {
    pub fn new() -> Self {
        let mut path = PathBuf::new();
        path.push(HOME);
        path.push(CHECK_DB);
        println!("Open monitor db at {:?}", &path);
        return Self {
            tree: sled::open(path).expect("Failed to open database for monitor"),
            miner_process_name: "miner",
            node_process_name: "libra-node"
        }
    }

    /// Persist Checks state
    pub fn write_db(&mut self, key: &str, value: &str) {
        self.tree.insert(key.as_bytes(), value).unwrap();
    }

    /// Read Checks state
    pub fn read_db(&self, key: &str) -> Option<IVec> {
        self.tree.get(key).unwrap()
    }

    /// nothing is configured yet, empty box
    pub fn is_clean_start() -> bool {
        // check to see no files are present
        let mut file = PathBuf::from(HOME);
        file.push("blocks/block_0.json"); //TODO change file name later
        file.exists()
    }

    /// the owner and operator accounts exist on chain
    pub fn accounts_exist_on_chain() -> bool {
        // check to see no files are present
        true
    }

    /// database is initialized
    pub fn database_bootstrapped() -> bool {
        true
    }

    /// Checks if node is synced
    pub fn node_is_synced() -> bool {
        Metadata::compare_from_config() < 1000
    }

    /// Check if node caught up, if so mark as caught up.
    pub fn check_sync(&mut self) -> bool {
        let sync = Check::node_is_synced();
        // let have_ever_synced = false;
        // assert never synced
        if self.has_never_synced() && sync {
            // mark as synced
            self.write_db(SYNC_KEY, "true");
        }
        sync  
    }

    /// check if the node has ever synced
    pub fn has_never_synced(&self) -> bool {
        match self.read_db(SYNC_KEY) {
            Some(state) => state!= b"true",
            None => false
        }
    }

    /// node started sync
    pub fn node_started_sync(&self) -> bool {
        match self.read_db(SYNC_KEY) {
            Some(state) => state!= b"true",
            None => false
        }
    }

    /// node is running
    pub fn node_is_running(&self) -> bool {
        let mut system = sysinfo::System::new_all();

        // First we update all information of our system struct.
        system.refresh_all();
        let ps = system.get_process_by_name(self.node_process_name );
        ps.len() > 0
    }

    /// miner is running
    pub fn miner_is_mining(&self) -> bool {
        let mut system = sysinfo::System::new_all();

        // First we update all information of our system struct.
        system.refresh_all();
        let ps = system.get_process_by_name(self.miner_process_name);
        ps.len() > 0
    }
}

/// Collects a pipeline of triggers for onboarding
pub fn onboarding_triggers() {
    // Validator just started setting up machine
    // if Check::is_clean_start() {
    //     management::fast_forward_db();
    //     return
    // }
    // // Restore was successful, can start syncing
    // if  Check::database_bootstrapped() && !Check::node_is_running() {
    //     management::start_node(management::NodeType::Validator);
    //     return
    // }
    //
    // // Validator account is created on chain, can start mining
    // if  Check::accounts_exist_on_chain()
    // && Check::database_bootstrapped()
    // && Check::node_is_running()
    // && Check::node_is_synced() // not
    // && !Check::miner_is_mining() {
    //     management::start_miner();
    //     return
    // }
    // // Node has caught up to rest of network
    // if  Check::node_is_synced() && !Check::miner_is_mining() {
    //     management::start_miner();
    //     return
    // }
    //
    //     // Node has caught up to rest of network
    // if  Check::node_is_synced() && !Check::miner_is_mining() {
    //     management::start_miner();
    //     return
    // }
}