#[derive(Parser)]
#[grammar = "../pests/create.pest"]
pub struct DDLParser;

use pest::Parser;
use pest::iterators::Pair;

pub fn parse_ddl(query: &str) -> Result<String, String> {
	let parsed = DDLParser::parse(Rule::ddl, query)
		.map_err(|e| e.to_string())?;

	let mut sql = String::new();
	for pair in parsed {
		match pair.as_rule() {
			Rule::create_schema => {
				let schema_name = pair.into_inner().next().unwrap().as_str();
				sql.push_str(&format!("CREATE SCHEMA IF NOT EXISTS {};\n", schema_name));
			},
			Rule::create_table => {
				sql.push_str(&process_create_table(pair));
			},
			_ => {}
		}
	}
	Ok(sql)
}

fn process_create_table(pair: Pair<Rule>) -> String {
	let mut sql = String::from("CREATE TABLE IF NOT EXISTS ");
	let mut inner_rules = pair.into_inner();

	// Extract table name
	let table_name = inner_rules.next().unwrap().as_str();
	sql.push_str(&format!("{} (", table_name));

	// Extract columns and constraints
	let columns = inner_rules.next().unwrap().into_inner();
	for column in columns {
		let column_inner = column.into_inner();
		let mut col_parts = vec![];
		for part in column_inner {
			col_parts.push(part.as_str());
		}
		let col_name = col_parts[0];
		let col_type = col_parts[1];

		// Handle column options (like String(100))
		let col_definition = if col_parts.len() > 2 {
			let col_options = &col_parts[2..].join(",");
			format!("{} {}({})", col_name, col_type_to_sql(col_type), col_options)
		} else {
			format!("{} {}", col_name, col_type_to_sql(col_type))
		};

		sql.push_str(&format!("{}, ", col_definition));
	}

	sql.pop(); // Remove the last comma
	sql.push_str(");\n");

	// Handle constraints (if any)
	for property in inner_rules {
		match property.as_rule() {
			Rule::constraints => {
				let constraints = property.into_inner();
				for constraint in constraints {
					sql.push_str(&process_constraint(constraint));
				}
			},
			_ => {}
		}
	}
	sql
}

fn col_type_to_sql(col_type: &str) -> &str {
	match col_type {
		"String" => "VARCHAR",
		"Integer" => "INTEGER",
		"Number" => "NUMERIC",
		"Text" => "TEXT",
		"Serial" => "SERIAL PRIMARY KEY",
		"Boolean" => "BOOLEAN",
		"DateTime" => "TIMESTAMP",
		_ => "TEXT", // Default to TEXT for unknown types
	}
}

fn process_constraint(pair: Pair<Rule>) -> String {
	let mut constraint_sql = String::new();
	let mut inner_rules = pair.into_inner();
	let constraint_type = inner_rules.next().unwrap().as_str();

	match constraint_type {
		"PRIMARY KEY" => {
			constraint_sql.push_str("PRIMARY KEY (");
			let columns = inner_rules.next().unwrap().as_str();
			constraint_sql.push_str(&format!("{})", columns));
		}
		"UNIQUE" => {
			constraint_sql.push_str("UNIQUE (");
			let columns = inner_rules.next().unwrap().as_str();
			constraint_sql.push_str(&format!("{})", columns));
		}
		"FOREIGN KEY" => {
			constraint_sql.push_str("FOREIGN KEY (");
			let columns = inner_rules.next().unwrap().as_str();
			constraint_sql.push_str(&format!("{}) REFERENCES ", columns));
			let foreign_table = inner_rules.next().unwrap().as_str();
			constraint_sql.push_str(&format!("{}", foreign_table));
		}
		_ => {}
	}

	constraint_sql.push_str(";\n");
	constraint_sql
}