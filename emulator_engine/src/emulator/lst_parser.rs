use super::bits::*;

use regex::Regex;
use std::collections::HashMap;

pub struct LstParser {
    pub address_info: HashMap<u16, usize>,
    pub content: Vec<(String, String)>,
    pub bytecode: Vec<u8>,
}

impl LstParser {
    pub fn from_bin_file(data: &[u8]) -> Self {
        let mut bytecode = Vec::from(data);

        Self {
            address_info: HashMap::new(),
            content: vec![(String::new(), String::from("<NO DEBUGGING INFO AVAILABLE>"))],
            bytecode,
        }
    }

    pub fn from_lst_file(data: String) -> Self {
        let command_rgx = Regex::new(r"^(\d{4})\s(\d{4})\s+\d+\s{2}(.*)$").unwrap();
        let info_rgx = Regex::new(r"^\s+\d+\s{2}(\s*\S.+)$").unwrap();

        let mut address_info = HashMap::new();
        let mut content = Vec::new();
        let mut bytecode = Vec::new();

        for line in data.lines() {
            let mut found = false;
            for cap in command_rgx.captures(line) {
                let address = u16::from_str_radix(&cap[1], 16).unwrap();
                let opcode = u16::from_str_radix(&cap[2], 16).unwrap();
                let info = String::from(&cap[3]);

                content.push((info, String::new()));
                address_info.insert(address, content.len() - 1);
                bytecode.push(get_high_byte(opcode));
                bytecode.push(get_low_byte(opcode));

                found = true;
            }

            if !found {
                for cap in info_rgx.captures(line) {
                    content.push((String::from(&cap[1]), String::new()));
                }
            }
        }

        Self {
            address_info,
            content,
            bytecode,
        }
    }
}
