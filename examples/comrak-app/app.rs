use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut args = std::env::args_os();
    let _bin = args.next();
    if let Some(path) = args.next() {
        let raw = std::fs::read_to_string(&path)?;
        render(&raw)?;
    }

    Ok(())
}

fn render(raw: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, raw, &comrak::ComrakOptions::default());

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    iter_nodes(root, &mut move |node| {
        let _ = writeln!(stdout, "{:?}", node.data.borrow().value);
    });

    Ok(())
}

fn iter_nodes<'a, F>(node: &'a comrak::nodes::AstNode<'a>, f: &mut F)
where
    F: FnMut(&'a comrak::nodes::AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}
