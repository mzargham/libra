// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0
//! Names of modules, functions, and types used by Diem System.

use diem_types::account_config;
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use once_cell::sync::Lazy;

// Data to resolve basic account and transaction flow functions and structs
/// The ModuleId for the diem writeset manager module
/// The ModuleId for the diem block module
pub static DIEM_BLOCK_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        Identifier::new("DiemBlock").unwrap(),
    )
});

//////// 0L ////////
// Oracle module
pub static ORACLE_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        ORACLE_MODULE_NAME.clone(),
    )
});
pub static UPGRADE_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        UPGRADE_MODULE_NAME.clone(),
    )
});
// Oracles
static ORACLE_MODULE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("Oracle").unwrap());
pub static CHECK_UPGRADE: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("check_upgrade").unwrap());
static UPGRADE_MODULE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("Upgrade").unwrap());
pub static RESET_PAYLOAD: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("reset_payload").unwrap());
//////// 0L end ////////    

// Names for special functions and structs
pub static SCRIPT_PROLOGUE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("script_prologue").unwrap());
pub static MODULE_PROLOGUE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("module_prologue").unwrap());
pub static WRITESET_PROLOGUE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("writeset_prologue").unwrap());
pub static WRITESET_EPILOGUE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("writeset_epilogue").unwrap());
pub static USER_EPILOGUE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("epilogue").unwrap());
pub static BLOCK_PROLOGUE: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("block_prologue").unwrap());
