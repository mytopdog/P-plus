use lib::Token;

fn is_var(c: char) -> bool {
	c == '_' || c == '$' || c.is_alphanumeric()
}

pub fn lex<'a>(contents: &'a String) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matched) in contents.match_indices(|c: char| !is_var(c)) {
		if last != index {
			result.push(&contents[last..index]);
		}
		
		result.push(matched);
		
		last = index + matched.len();
	}
	
	if last < contents.len() {
		result.push(&contents[last..]);
	}
	
	result
}

pub fn lex2(tokens: Vec<&str>) -> Vec<Token> {
	let mut res: Vec<Token> = Vec::new();
	let mut string = Token {
		val: String::from(""),
		t: "str1"
	};
	
	let mut in_str = false;
	let mut in_str2 = false;
	let mut escaping = false;
	let mut ignoring = false;
	let mut ignoring2 = false;
	let mut possible_comment = false;
	
	let mut num_pos = 0;
	
	for item in tokens {
		if ignoring {
			if item == "\r" || item == "\n" {
				res.push(Token {val: item.to_string(), t: "whitespace"});
				
				ignoring = false;
			}
		} else if ignoring2 {
			if possible_comment {
				if item == "/" {
					ignoring2 = false;
				}
				
				possible_comment = false;
			}
			
			if item == "*" {
				possible_comment = true;
			}
		} else {
			if possible_comment {
				if item == "/" {
					ignoring = true;
					possible_comment = false;
					
					continue;
				} else if item == "*" {
					ignoring2 = true;
					possible_comment = false;
					
					continue;
				} else {
					possible_comment = false;
					
					string.val = String::from("/");
					string.t = "operator";
					
					res.push(string.clone());
					string.val = String::from("");
				}
			}
			
			if escaping {
				if item == "0" || item == "n" { // Null and newlines
					string.val += "\\";
				}
				string.val += item;
				
				escaping = false;
			} else if in_str {
				if item == "\"" {
					res.push(string.clone());
					string.val = String::from("");
					in_str = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					string.val += item;
				}
			} else if in_str2 {
				if item == "'" {
					res.push(string.clone());
					string.val = String::from("");
					in_str2 = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					string.val += item;
				}
			} else if item == "\"" {
				string.t = "str1";
				in_str = true;
			} else if item == "'" {
				string.t = "str2";
				in_str2 = true;
			} else {
				if num_pos > 0 && (item == "." || num_pos == 2) {
					string.val += item;
					if num_pos == 2 {
						res.push(string.clone());
						string.val = String::from("");
						
						num_pos = 0;
					} else {
						num_pos = 2;
					}
					
					continue;
				} else if num_pos == 1 {
					res.push(string.clone());
					string.val = String::from("");
					
					num_pos = 0;
				}
				
				if item == "/" {
					possible_comment = true;
				} else if item.parse::<u64>().is_ok() {
					string.val = item.to_string();
					string.t = "number";
					
					num_pos = 1;
				} else {
					string.val = item.to_string();
					string.t = match item {
						"+" | "-" | "*" | "/" | "%" | "=" | "&" | "|" | "^" | "<" | ">" | "!" | "~" | "?" | ":" | "." | "," | "@" | ";" => "operator",
						"{" | "}" | "[" | "]" | "(" | ")" => "group operator",
						"array" | "bool" | "chan" | "char" | "const" | "fraction" | "func" | "heap" | "int" | "list" | "number" | "only" | "pointer" | "register" | "signed" | "stack" | "unique" | "unsigned" | "void" | "volatile" => "type",
						"as" | "async" | "break" | "continue" | "else" | "export" | "foreach" | "from" | "goto" | "if" | "import" | "in" | "match" | "receive" | "repeat" | "return" | "select" | "send" | "to" | "type" | "until" | "when" | "while" => "reserved",
						"false" | "true" => "literal",
						"\n" | "\r" | "\t" | " " => "whitespace",
						_ => "variable"
					};
					
					res.push(string.clone());
					string.val = String::from("");
				}
			}
		}
	}
	
	res
}