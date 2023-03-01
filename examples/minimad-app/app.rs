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
    let text = minimad::parse_text(raw, minimad::Options::default());

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for line in &text.lines {
        let _ = writeln!(stdout, "{:?}", line);
    }

    Ok(())
}
