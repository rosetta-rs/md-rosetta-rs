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
    let text = minimad::parse_text(raw, minimad::Options::default());

    for line in &text.lines {
        let _ = writeln!(writer, "{:?}", line);
    }

    Ok(())
}
