// https://playground.oxc.rs/#eNrNVstu2zoQ/RVGuAsb8GOvXDc3z+YCebhxgi7iwKCkkUyEIlWSsuOm/vcORdqSHbntsoYBiZzDOfPijN6DOAgDlhdSGfJOSg3nlPOIxq+9aiGFgTdD1iRVMifTQAGNzTSYiu2RxznkUAO0WXFI+kpYUAM2LiPO9JyJbEyVEaBOiwKooiJuHP5HAaer4bj8/j2X56VSIMyTBnWlaJbj+yBTtJh/4zsGfNB8Rxdn6ECmZCmSXqv8Sipw8gPkX0pQqyZdgxAt/xCXwWCI/1qyY2GMUTNwuUAPLnNmDKgeyekroGt+PSkjHStWGCbFjlLOoqE/7pDOFrMqgNyC1jQD/QAxgwUkZIRct5PPs8f72el4HOLKgDYh0Uah92R9TE6fHq9nt/cXlzeVFLWEyMNlxjBf5IfNH8tEWeBiTdaWaTjEDDNN7D/PS0MjDkfkQhIhDZkpyOUCZkQqMovnVGT4bubgGfGIsEsNJGXAEz3w+oAIatgCCC0KIgAS7WFGElmAQncR6t3EoE4KdDFlsU+gdXQqCLFl+vSArjg6uyXo4hpYNkevRZlHGC/cRP/kjRTZGccDTfRG8HXODEZiV9Cu5ytLzLy5KUt1g/v3aarB6JBgshIp+MpDnl8IuvzsFj2/+WIPosfs90fbTmLU/NUJf3WvfDzOPregdu6IB14dANaXpeHvIfCu1i34fgFKsWQbZKw1UXKOD0RCygQgviq4ndKe4JUhI6vph0u5/W3KFsPQj+vr5oTYtNTEYAH9f9FM6EZU7+5SO0Th3PgjUNhWmG1eeaNtmwxdt3R76G2rW9qgsr4uo9opXcYxRmSv1u0vpYy3bNM4xhSYCS+zfZE3v02EDQu7+zln8et+9H5tbMbSRgr+CmurToMGby3NqcCK6keMc4sLjtto3UksKm18i6q6tvbNFxvPx17+734f/tTpVuz/Gd2HN6wQ08fixxbZJxHEFMuQyJRkXEaU2946FfX7oJ21ZdcaitrthHH2ol4/RiJ4lHfVCW8aajg8b1rsb6FDl1r4QCT7RJ0uGX1yheJw77ZfPWCgWfWdMGp8V3TqgVnpJ9gCTalE8zuk41LayR1DuNMbaq6qIlLSaXAdjUaYd+fLNOh63U3wUWcaLCF6ZVi9dlYtmUjkstvF26rkEifTklzazCHsThKHJDihHQ51NpV13C6hmlCx6p4MHP5k4C2/piLhoPTJwE69ERq3bRIHCP2QxLlqDzbojve4PNU+kyUaFFIbH7JNDL2edc89n+uYvVRbKMd7EPQCGYTvgSqFfeiVMPQtCI0qoRfgHTKbdx3jzN4uVnkk+WZlcBDpVKo8CFPKNax7Ad5o7MOoEd+tls37FrrZiGUCGVhuXORMYJ+tRcIoya+4XFrTcLZEUqMJjmO9/gm/6s/z

#![allow(clippy::print_stdout)]

use std::io::Read;
use oxc_allocator::{Allocator, CloneIn};
use oxc_ast::ast::{  PropertyKey, TSSignature, TSType, TSTypeAnnotation};
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use oxc_ast::{
    ast::TSTypeLiteral,
    visit::walk,
    Visit,
};



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
        panicked, // Parser encountered an error it couldn't recover from
        ..
    }  = Parser::new(&allocator, &contents, source_type).parse();

    assert!(!program.body.is_empty());
    assert!(!panicked);

    let mut ast_pass = ASTPass::default();
    ast_pass.visit_program(&program);

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

        let name_ts = it.id.name.to_string();
        self.swift_objects.push(name_ts);

        // If the annotation is a union type
        if let TSType::TSUnionType(union_type) = &it.type_annotation {
            println!("union type");
            // Loop through the union types
            for type_ in union_type.types.iter() {
                print_ts_sub_type(it.id.name.to_string(), type_);
            }
        }

        // It is an object type
        if let TSType::TSTypeLiteral(literal) = &it.type_annotation {
            println!("literal type");
            print_ts_root_type(it.id.name.to_string(), literal);            
        }

        walk::walk_ts_type_alias_declaration(self, it);
    }
}

fn print_ts_root_type(name: String, ts_type: &TSTypeLiteral) {
    // Create a mutable string which we append to
    let mut swift_object = String::new();

    // struct AppSpecificPartner: Codable 
    swift_object.push_str( &format!("struct {}: Codable {{\n  ", name));

    // Loop through the members of the object type
    for member in ts_type.members.iter() {
        if let TSSignature::TSPropertySignature(property) = member {
            // the key could be an expression, or identifier
            let key = match &property.key {
                PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                _ => todo!(),
            };

            let arena = Allocator::default();
            let ts_type_annotation = match &property.type_annotation {
                Some(ts_type_annotation) => ts_type_annotation,
                None => todo!(),
            };

            let our_type = ts_type_annotation.clone_in(&arena);
            match our_type.unbox() {
                TSTypeAnnotation { type_annotation: TSType::TSAnyKeyword(_), .. } => swift_object.push_str(&format!("let {}: AnyObject?", key)),
                TSTypeAnnotation { type_annotation: TSType::TSStringKeyword(_), .. } => swift_object.push_str(&format!("let {}: String?", key)),
                _ => ()
            }
            swift_object.push_str("\n");
        }
    }

    // Remove the last comma from the string
    swift_object.push('}');

    // Print the string
    print!("object: {:?}", swift_object);
}

fn print_ts_sub_type(_root_name: String, _ts_type: &TSType) {
}