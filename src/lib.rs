// #![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]
// #![allow(
// clippy::enum_variant_names,
// clippy::large_enum_variant,
// clippy::module_name_repetitions,
// clippy::similar_names
// )]
//

mod alias;
mod arch;
mod archive;
mod choose_version_for_user_input;
pub mod cli;
pub mod commands;
pub mod config;
mod current_version;
mod directory_portal;
mod downloader;
mod fs;
mod http;
mod installed_versions;
mod lts;
mod package_json;
mod path_ext;
mod remote_node_index;
pub mod shell;
mod system_info;
mod system_version;
pub mod user_version;
pub mod user_version_reader;
mod version;
mod version_file_strategy;
mod version_files;

#[macro_use]
mod log_level;
mod default_version;
mod directories;