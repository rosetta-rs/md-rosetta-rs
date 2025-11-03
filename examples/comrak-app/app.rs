use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut args = std::env::args_os();
    let _bin = args.next();
    let path = args.next().expect("no markdown path given");
    let raw = std::fs::read_to_string(path)?;
    #[cfg(debug_assertions)]
    {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        render(&mut stdout, &raw)?;
    }
    #[cfg(not(debug_assertions))]
    {
        let mut buffer = Vec::new();
        render(&mut buffer, &raw)?;
        std::hint::black_box(buffer);
    }
    Ok(())
}

fn render(
    writer: &mut dyn Write,
    raw: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, raw, &comrak::Options::default());

    iter_nodes(root, &mut move |node| {
        let _ = writeln!(writer, "{:?}", node.data.borrow().value);
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
