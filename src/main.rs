#![allow(clippy::print_stdout)]

use std::{io::Read, process::exit};
use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use oxc_ast::{
    ast::{Class, Function, TSImportType, Statement},
    visit::walk,
    Visit,
};
use oxc_syntax::scope::ScopeFlags;


fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    println!("path: {:?}",  path);

    let mut file = std::fs::File::open
    (&path).expect("could not open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read file");

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap();
    let ParserReturn {
        program,  // AST
        errors,   // Syntax errors
        panicked, // Parser encountered an error it couldn't recover from
        ..
    }  = Parser::new(&allocator, &contents, source_type).parse();

    assert!(!program.body.is_empty());
    assert!(!panicked);

    // println!("program: {:?}", program.body);

    let mut ast_pass = CountASTNodes::default();
    ast_pass.visit_program(&program);



    println!("functions: {}", ast_pass.functions);

    println!("classes: {}", ast_pass.classes);

    println!("ts_import_types: {}", ast_pass.ts_import_types);
    println!("statements: {}", ast_pass.statements);
    

}


#[derive(Debug, Default)]
struct CountASTNodes {
    functions: usize,
    classes: usize,
    ts_import_types: usize,
    statements: usize,
}


impl<'a> Visit<'a> for CountASTNodes {
    fn visit_ts_type_alias_declaration(&mut self, it: &oxc_ast::ast::TSTypeAliasDeclaration<'a>) {
        println!("ts_type_alias_declaration: {:?}", it);
        println!("");println!("");
        walk::walk_ts_type_alias_declaration(self, it);
    }
}