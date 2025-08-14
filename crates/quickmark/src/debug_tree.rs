use std::env;
use tree_sitter::{Node, Parser, Language};

extern "C" {
    fn tree_sitter_markdown() -> Language;
}

fn print_tree(node: Node, source: &str, indent: usize) {
    let indentation = "  ".repeat(indent);
    let text = &source[node.start_byte()..node.end_byte()];
    let text_preview = if text.len() > 50 {
        format!("{}...", &text[..47])
    } else {
        text.to_string()
    };
    println!("{}{}@{}:{} = {:?}", indentation, node.kind(), node.start_byte(), node.end_byte(), text_preview);
    
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            print_tree(cursor.node(), source, indent + 1);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.md>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let content = std::fs::read_to_string(file_path)?;
    
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_markdown() };
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&content, None).unwrap();
    print_tree(tree.root_node(), &content, 0);
    
    Ok(())
}