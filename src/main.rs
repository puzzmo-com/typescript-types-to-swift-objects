#![allow(clippy::print_stdout)]

use std::{io::Read, process::exit};
use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, TSType};
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

    let mut ast_pass = ASTPass::default();
    ast_pass.visit_program(&program);

    println!("swift_objects: {:?}", ast_pass.swift_objects);

}


#[derive(Debug, Default)]
struct ASTPass {
    // a list of strings the names of the Swift objects we're going to generate
    swift_objects: Vec<String>,
    
}


impl<'a> Visit<'a> for ASTPass {
    fn visit_ts_type_alias_declaration(&mut self, it: &oxc_ast::ast::TSTypeAliasDeclaration<'a>) {
        println!("");
        println!("");
        println!("type_alias_declaration: {:?}", it.id.name);

        let name_ts = it.id.name.to_string();
        self.swift_objects.push(name_ts);

        // If the annotation is a union type
        if let TSType::TSUnionType(union_type) = &it.type_annotation {
            println!("union type");
            // Loop through the union types
            for type_ in union_type.types.iter() {
                print_ts_subtype(it.id.name.to_string(), type_);
            }
        }

        // It is an object type
        if let TSType::TSTypeLiteral(object) = &it.type_annotation {
            println!("literal type");
            
        }


        walk::walk_ts_type_alias_declaration(self, it);
    }
}

fn print_ts_subtype(name: String, ts_type: &TSType) {
//     match ts_type {
//         TSType::TSNumberKeyword => println!("number"),
//         TSType::TSStringKeyword => println!("string"),
//         TSType::TSBooleanKeyword => println!("boolean"),
//         TSType::TSAnyKeyword => println!("any"),
//         TSType::TSUnknownKeyword => println!("unknown"),
//         TSType::TSNullKeyword => println!("null"),
//         TSType::TSUndefinedKeyword(_) => println!("undefined"),
//         // TSType::TSNeverKeyword => println!("never"),
//         // TSType::TSObjectKeyword => println!("object"),
       
//         TSType::TSUnionType(union_type) => {
//             println!("union type");
//             for type_ in union_type.types.iter() {
//                 print_ts_type(type_);
//             }
//         }
//         TSType::TSIntersectionType(intersection_type) => {
//             println!("intersection type");
//             for type_ in intersection_type.types.iter() {
//                 print_ts_type(type_);
//             }
//         }
//         TSType::TSTypeLiteral(type_literal) => {
//             println!("type literal");
//             for member in type_literal.members.iter() {
//                 println!("member: {:?}", member);
//             }
//         }
//         // TSType::TSFunctionType(function_type) => {
//         //     println!("function type");
//         //     for param in function_type.parameters.iter() {
//         //         println!("param: {:?}", param);
//         //     }
//         //     println!("return_type: {:?}", function_type.return_type);
//         // }
   
   
//         TSType::TSIndexedAccessType(indexed_access_type) => {
//             println!("indexed access type");
//             println!("object_type: {:?}", indexed_access_type.object_type);
//             println!("index_type: {:?}", indexed_access_type.index_type);
//         }
      
        
//         TSType::TSTypeQuery(type_query) => {
//             println!("type query");
//             println!("expr_name: {:?}", type_query.expr_name);
//         }
//         TSType::TSQualifiedName(qualified_name) => {
//             println!("qualified name");
//             println!("left: {:?}", qualified_name.left);
//             println!("right: {:?}", qualified_name.right);
//         }
        
//         TSType::TSIndexedAccessType(indexed_access_type) => {
//             println!("indexed access type");
//             println!("object_type: {:?}", indexed_access_type.object_type);
//             println!("index_type: {:?}", indexed_access_type.index_type);
//         }
      
//         TSType::TSTypeLiteral(type_literal) => {
//             println!("type literal");
//             for member in type_literal.members.iter() {
//                 println!("member: {:?}", member);
//             }
//         }
      
//         // TSType::TSArrayType(array_type) => {
//         //     println!("array type");
//         //     print_ts_type(&*array_type.element_type);
//         // }
//         TSType::TSUnionType(union_type) => {
//             println!("union type");
//             for type_ in union_type.types.iter() {
//                 print_ts_type(type_);
//             }
//         }
//     }
}