use crate::parser::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{self, ErrorKind, Read},
};

type Record = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
struct Definition {
    total: u32,
    data: HashMap<String, String>,
}

type DefinitionRegister = HashMap<String, Definition>;
type DefinitionRecord = HashMap<String, String>;

pub struct Generator {
    definition: String,
}

impl Generator {
    pub fn new(definition: String) -> Self {
        Generator { definition }
    }

    fn read_file(&self) -> Result<String, io::Error> {
        let cwd_path = current_dir()?;
        let mut file_content: String = String::new();

        let mut def_file =
            File::open(cwd_path.join(self.definition.clone())).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    panic!("File definitions not found: {}", error);
                } else {
                    panic!("Error on open file: {}", error);
                }
            });

        def_file.read_to_string(&mut file_content)?;

        Ok(file_content)
    }

    fn transform(&self) -> DefinitionRegister {
        let def_file_str = self.read_file().unwrap();
        let definitions: DefinitionRegister =
            serde_json::from_str(&def_file_str).expect("Error on transform data");

        definitions
    }

    pub fn gen(&self) -> HashMap<String, Vec<DefinitionRecord>> {
        let definitions = self.transform();
        let mut registers: HashMap<String, Vec<DefinitionRecord>> = HashMap::new();

        for rec_key in definitions.keys() {
            if let Some(def) = definitions.get(rec_key) {
                let mut records: Vec<Record> = Vec::new();

                for _ in 0..def.total {
                    let mut record: Record = HashMap::new();

                    for reg_key in def.data.keys() {
                        if let Some(def_data) = def.data.get(reg_key) {
                            record.insert(reg_key.to_string(), Parser::parse(def_data.as_str()));
                        }
                    }

                    records.push(record);
                }

                registers.insert(rec_key.to_string(), records);
            }
        }
        println!("{:?}", registers);

        registers
    }
}
