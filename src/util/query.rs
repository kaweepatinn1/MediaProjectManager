extern crate fs_extra;
use core::panic;

use fs_extra::dir::get_dir_content;
use crate::Query;
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum QueryResult{
    GeneralResult(GeneralResult),
    FullResult(FullResult),
}

impl QueryResult{
    fn general_result(self) -> GeneralResult{
        if let QueryResult::GeneralResult(g) = self {
            g
        } else{
            panic!("Query Result does not contain a general result!");
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneralResult{
    query_of_folder : String,
    file_count : usize,
    total_size : u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct FullResult{

}

pub fn query(query: Query, config: Config, write: bool) {
    match query {
        Query::General => {query_general(config, write);},
        Query::All => {query_all(write);},
        Query::Folder(folder) => {query_by_folder(folder, write);},
        Query::FolderRecursive(folder) => {query_folder_recursive(folder, write);},
        Query::Day(day) => {query_by_day(day, write);},
        Query::None => {}, // Unreachable
    }
}

pub fn query_general(config: Config, write: bool) {
    let path = &format!("./{}",config.setup.name);
    // let metadata = fs::metadata(path);
    let files = get_dir_content(path).expect("Could not get directory content!");
    let result = QueryResult::GeneralResult( 
        GeneralResult{
            query_of_folder: config.setup.name,
            file_count: files.files.len(), 
            total_size: files.dir_size},
    );
    if write {
        let text = toml::to_string(&result.general_result()).expect("Could not serialize query result!");
        std::fs::write("general_query.txt", text).expect("Failed to write query result!");
    }
}

pub fn query_all(write: bool) {

}

pub fn query_by_folder(folder : Vec<String>, write: bool) {
    dbg!(folder);
}

pub fn query_folder_recursive(folder : String, write: bool) {
    
}

pub fn query_by_day(day : usize, write: bool) {
    
}