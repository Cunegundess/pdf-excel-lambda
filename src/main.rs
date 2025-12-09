use pdf::content::Op;
use pdf::file::FileOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_path = "ponto.pdf";
    let file = FileOptions::cached().open(pdf_path)?;

    println!("📄 Extraindo texto de {} páginas...\n", file.num_pages());

    let mut extracted = String::new();

    for page_num in 0..file.num_pages() {
        let page = file.get_page(page_num)?;

        if let Some(content) = page.contents.as_ref() {
            let ops = content.operations(&file.resolver())?;

            for op in ops {
                match op {
                    Op::TextDraw { text } => {
                        extracted.push_str(&text.to_string()?);
                        extracted.push(' ');
                    }
                    Op::TextDrawAdjusted { array } => {
                        for item in array {
                            if let pdf::content::TextDrawAdjusted::Text(t) = item {
                                extracted.push_str(&t.to_string()?);
                            }
                        }
                        extracted.push(' ');
                    }
                    _ => {}
                }
            }
        }

        extracted.push_str(&format!("\n--- Fim da página {} ---\n", page_num + 1));
    }

    println!("Texto extraído:\n{}", extracted.trim());
    Ok(())
}

