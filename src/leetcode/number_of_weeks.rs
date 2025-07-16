
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};
use swc_ecma_ast::Module;
use std::fs;

fn main() {
    // Load the TypeScript file
    let filename = "path/to/your/file.ts";
    let ts_code = fs::read_to_string(filename).expect("Unable to read file");

    // Create a SourceMap
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom(filename.to_string()), ts_code);

    // Create a parser for TypeScript
    let syntax = Syntax::Typescript(TsConfig {
        tsx: false, // Set to true if you are parsing TSX (TypeScript + JSX)
        dynamic_import: true,
        ..Default::default()
    });

    let mut parser = Parser::new(syntax, StringInput::from(&*fm), None);

    // Parse the file into an AST
    let module: Module = parser.parse_module().expect("Failed to parse module");

    // Now `module` contains the ESTree representation of the TypeScript file
    println!("{:#?}", module);
}
