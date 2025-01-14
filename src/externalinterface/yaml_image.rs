extern crate yaml_rust;

use std::path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use crate::entity::{
    self,
    attr,
    data,
    entry
};
use crate::interfaceadapter::worker;

#[derive(Debug)]
struct YAMLImageStruct {
    entry: path::PathBuf,
    attr: path::PathBuf,
    data: path::PathBuf
}

const ATTR:         &str = "attr";
const DATA:         &str = "data";
const ENTRY:        &str = "entry";
const INO:          &str = "ino";
const FILE_TYPE:    &str = "file-type";
const PARENT_INO:   &str = "parent-ino";
const NAME:         &str = "name";
const METADATA:     &str = "metadata";
const USER:         &str = "user";
const GROUP:        &str = "group";
const TIME:         &str = "time";
const FILES:        &str = "files";
const SIZE:         &str = "size";

const ATTR_DEFAULT_PATH: &str = "/etc/attr.yaml";
const ENTRY_DEFAULT_PATH: &str = "/etc/entry.yaml";
const DATA_DEFAULT_PATH: &str = "/etc/data.yaml";

const DIRECTORY: u64 = 0;
const TXTFILE: u64 = 1;

pub fn new() -> impl worker::File {
    YAMLImageStruct{
        attr: path::PathBuf::from(ATTR_DEFAULT_PATH),
        entry: path::PathBuf::from(ENTRY_DEFAULT_PATH),
        data: path::PathBuf::from(DATA_DEFAULT_PATH)
    }
}

impl worker::File for YAMLImageStruct {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct, ()> {
        self.load_image(path);
        let entries = match self.load_entry() {
            Ok(entries) => entries,
            Err(_) => return Err(())
        };
        let attrs = match self.load_attr() {
            Ok(attrs) => attrs,
            Err(_) => return Err(())
        };
		
		println!("{:?}", entries);
		println!("{:?}", attrs);
        return Ok(entity::new(attrs, entries, HashMap::new()));
    }

    fn attr_from_ino(&self, path: &path::Path, ino: u64) -> Result<attr::Attr, ()> {
        Ok(attr::new(1, 1, String::from("this is name"), attr::FileType::Directory))
    }

    fn data_from_ino(&self, path: &path::Path, ino: u64) -> Result<data::Data, ()> {
        Ok(data::new(1, String::from("this is data")))
    }

    fn entry_from_ino(&self, path: &path::Path, ino: u64) -> Result<entry::Entry, ()> {
        Ok(entry::new(1, 1))
    }
}

impl YAMLImageStruct {
    fn load_image(&mut self, path: &path::Path) -> Result<(), ()> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(())
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {},
            Err(e) => return Err(())
        };
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(())
        };

        self.attr = match &docs[0][ATTR] {
            Yaml::String(s) => path::PathBuf::from(s),
            _ => path::PathBuf::from(ATTR_DEFAULT_PATH)
        };

        self.entry = match &docs[0][ENTRY] {
            Yaml::String(s) => path::PathBuf::from(s),
            _ => path::PathBuf::from(ENTRY_DEFAULT_PATH)
        };

        self.data = match &docs[0][DATA] {
            Yaml::String(s) => path::PathBuf::from(s),
            _ => path::PathBuf::from(DATA_DEFAULT_PATH)
        };

        return Ok(());
    }

    fn load_entry(&self) -> Result<HashMap<u64, Vec<entry::Entry>>, ()> {
        let mut file = match File::open(&self.entry) {
            Ok(file) => file,
            Err(e) => {
				return Err(());
			}
        };

        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(e) => return Err(())
        }
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(())
        };
        let mut entrie_hash = HashMap::new();

        for entry_data in docs[0].as_vec().unwrap() {
            let mut entries = Vec::new();
            let ino = match &entry_data[INO] {
                Yaml::Integer(i) => *i as u64,
                _ => return Err(())
            };

            match &entry_data[FILES] {
                Yaml::Array(child_inos_data) => {
                    for child_ino_data in child_inos_data {
                        let child_ino = match child_ino_data {
                            Yaml::Integer(i) => *i as u64,
                            _ => return Err(())
                        };

                        entries.push(entry::new(ino, child_ino));
                    }
                },
                _ => {}
            }

            entrie_hash.insert(ino, entries);
        }

        return Ok(entrie_hash);
    }

    fn load_attr(&self) -> Result<HashMap<u64, attr::Attr>, ()> {
        let mut file = match File::open(&self.attr) {
            Ok(file) => file,
            Err(e) => return Err(())
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(e) => return Err(())
        };
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(())
        };
        let mut attrs_hash = HashMap::new();
        
        for attr_data in docs[0].as_vec().unwrap() {
            let ino = match &attr_data[INO] {
                Yaml::Integer(i) => *i as u64,
                _ => return Err(())
            };
            let name = match &attr_data[NAME] {
                Yaml::String(s) => s.clone(),
                _ => return Err(())
            };
            let file_type = match &attr_data[FILE_TYPE] {
                Yaml::Integer(i) =>{
                    match *i as u64 {
                        DIRECTORY => attr::FileType::Directory,
                        TXTFILE => attr::FileType::TextFile,
                        _ => attr::FileType::TextFile
                    }
                },
                _ => return Err(())
            };
            let size = match &attr_data[SIZE] {
                Yaml::Integer(i) => {
                    *i as u64
                },
                _ => return Err(())
            };

            attrs_hash.insert(ino, attr::new(ino, size, name, file_type));
        }

        return Ok(attrs_hash);
    }
}
