use tree_sitter::{Parser, Language, Node};

extern "C" { fn tree_sitter_markdown() -> Language; }

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
    cursor.goto_first_child();
    loop {
        print_tree(cursor.node(), source, indent + 1);
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn main() {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_markdown() };
    parser.set_language(&language).unwrap();

    let source_code = std::fs::read_to_string("/tmp/debug_md040.md").unwrap();
    let tree = parser.parse(&source_code, None).unwrap();
    
    print_tree(tree.root_node(), &source_code, 0);
}