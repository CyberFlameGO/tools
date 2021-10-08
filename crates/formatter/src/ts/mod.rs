use rslint_parser::SyntaxToken;

use crate::{format_elements, token, FormatElement};

mod array;
mod arrow_function;
mod declarators;
mod expressions;
mod literal;
mod name;
mod name_ref;
mod paramter_list;
mod patterns;
mod script;
mod spread;
mod statements;
mod var_decl;

pub fn format_syntax_token(syntax_token: SyntaxToken) -> FormatElement {
	format_elements!(token(syntax_token.text().as_str()))
}

#[cfg(test)]
mod test {
	use rslint_parser::{ast::Script, parse_text, AstNode};

	use crate::{format_element, FormatOptions, ToFormatElement};

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_element(&child.to_format_element(), FormatOptions::default());
		assert_eq!(result.code(), "let v = (value, second_value) => true;");
	}

	#[test]
	fn function_block() {
		let src = r#"function foo() { return 'something' }"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_element(&child.to_format_element(), FormatOptions::default());
		assert_eq!(result.code(), r#"function foo() {return "something";}"#);
	}

	#[test]
	fn array() {
		let src = r#"let users = [   'john', 'chandler', true ]"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_element(&child.to_format_element(), FormatOptions::default());
		assert_eq!(result.code(), r#"let users = ["john", "chandler", true];"#);
	}

	#[test]
	fn poc() {
		let src = r#"function foo() { let var1 = [true, false]
	let broken = [   45, 54]
	let var2 = (var1, var2) => {}
}"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_element(&child.to_format_element(), FormatOptions::default());
		assert_eq!(
			result.code(),
			r#"function foo() {
	let var1 = [true, false];
	let broken = [45, 54];
	let var2 = (var1, var2) => {};
}"#
		);
	}
}