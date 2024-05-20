/*
VALKYRIE by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing serde's
/// "Deserialize" trait
/// to deserialize data
/// into a Rust data structure.
use serde::Deserialize;

/// Importing Valkyrie's
/// error-handling structure.
use super::err::ValkyrieErr;

/// Importing the stanndard HashMap
/// data structure.
use std::collections::HashMap;

/// A data structure
/// to hold the information
/// of the project manifest.
[#derive(Deserialize, Debug)]
pub struct Project{
    pub name: String,
    pub imports: HashMap<String, String>,
    pub permissions: Vec<String>
}

/// This function receieves a string
/// containing a project manifest's contents.
/// Thius string is parsed into the "Project"
/// data structure. If this fails, an error is
/// returned.
pub fn parse_project(
    manifest: &String
) -> Result<Project, ValkyrieErr> {
    let project: Project = match serde_json::from_str(manifest){
        Ok(project) => project,
        Err(e) => return Err::<Project, ValkyrieErr>(ValkyrieErr::new(&e.to_string()));
    };
    Ok(project)
}
