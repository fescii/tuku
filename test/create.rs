#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_schema() {
		let result = parse_ddl("CREATE SCHEMA my_schema;").unwrap();
		assert_eq!(result, "CREATE SCHEMA my_schema;");
	}

	#[test]
	fn test_create_table() {
		let result = parse_ddl("CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255) NOT NULL);").unwrap();
		assert_eq!(result, "CREATE TABLE users (id INT, name VARCHAR(255));");
	}
}
