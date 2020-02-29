use super::bits::*;

use regex::Regex;
use std::collections::HashMap;

pub struct ParseResult {
    pub address_info: HashMap<u16, usize>,
    pub content: Vec<(String, String)>,
    pub bytecode: Vec<u8>,
}

pub fn parse_bin_file(data: &[u8]) -> ParseResult {
    let bytecode = Vec::from(data);

    ParseResult {
        address_info: HashMap::new(),
        content: vec![(String::new(), String::from("<NO DEBUGGING INFO AVAILABLE>"))],
        bytecode,
    }
}

pub fn parse_lst_file(data: &str) -> ParseResult {
    // TODO: Better regexes
    let command_rgx = Regex::new(r"^(\d{4})\s(\d{4})\s+\d+\s{2}(\S*)\s+(.*)$").unwrap();
    let info_rgx = Regex::new(r"^[^\S\r\n]{20}\d{5}[^\S\r\n]{2}(\S*)[^\S\r\n]+(.*)$").unwrap();

    let mut address_info = HashMap::new();
    let mut content = Vec::new();
    let mut bytecode = Vec::new();

    for line in data.lines() {
        let mut found = false;
        for cap in command_rgx.captures(line) {
            let index = u16::from_str_radix(&cap[1], 16).unwrap();
            let opcode = u16::from_str_radix(&cap[2], 16).unwrap();
            let label = String::from(&cap[3]);
            let info = String::from(&cap[4]);

            address_info.insert(index, content.len());
            content.push((label, info));
            bytecode.push(get_high_byte(opcode));
            bytecode.push(get_low_byte(opcode));
            found = true;
        }

        if !found {
            for cap in info_rgx.captures(line) {
                if &cap[1] != "" || &cap[2] != "" {
                    content.push((String::from(&cap[1]), String::from(&cap[2])));
                }
            }
        }
    }

    ParseResult {
        address_info,
        content,
        bytecode,
    }
}
