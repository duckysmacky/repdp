use std::io::Write;
use std::any::{Any, TypeId};
use std::fmt::{Display, Formatter};
use std::{fmt, io};
use crate::demo::Demo;

pub mod data {
	const VERSION: &str = "0.0.1";
}

#[derive(Debug)]
pub struct DumperInfo {
	dumper_version: String,
	file_name: String,
	game_name: String
}

impl Display for DumperInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Generated by IIPDP v{}", self.dumper_version)?;
		write!(f, "Demo file: {}", self.file_name)?;
		write!(f, "Presumed game: {}", self.game_name)
	}
}

// check if type of value is a primitive
// not used but might later
fn is_primitive(value: &dyn Any) -> bool {
    vec![
		TypeId::of::<String>(),
		TypeId::of::<u8>(),
		TypeId::of::<i32>(),
		TypeId::of::<u64>(),
		TypeId::of::<i16>(),
		TypeId::of::<char>(),
	].contains(&value.type_id())
}

pub fn dump_demo(demo: &Demo, out: &mut dyn Write) -> io::Result<()> {
	// TODO: write!(out, "{}", dumper_info)?;
	write!(out, "{}", demo)?;
	out.flush()
}

/// Returns a HashMap consisting of string field-value pairs from a given struct
#[macro_export]
macro_rules! get_fields {
	($obj:ident) => {{
		let debug = format!("{:#?}", $obj);
		println!("{}", debug);
		let debug = debug.split('\n').collect::<Vec<&str>>();
		// map over every field-value pair except first and last lines
		let iter = debug[1..debug.len() - 1].iter()
			.map(|line| {
				let line = line.trim();
				// split a single line into a field and its value
				let (field, val) = line.split_at(line.find(" ").expect("Invalid debug format"));
				// cut unnecessary characters and convert both to String
				let field = crate::dumper::prettify_field(field);
				let val = String::from(&val[1..val.len() - 1]).replace('\"', "");
				(field, val)
			});
		let map: std::collections::HashMap<String, String> = std::collections::HashMap::from_iter(iter);
		map
	}};
}

/// Coverts a struct field name from snake case to camel case
pub fn prettify_field(field: &str) -> String {
	let mut pretty_field = String::from(field);
	while let Some(index) = pretty_field.find('_') {
		let upper = pretty_field.chars().nth(index + 1).unwrap().to_ascii_uppercase();
		pretty_field.replace_range(index..index + 2, &format!(" {}", upper));
	}
	let mut chars = pretty_field.chars();
	chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}