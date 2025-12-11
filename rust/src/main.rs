use pdfium_render::prelude::*;

#[derive(serde::Serialize)]
pub struct JSONResponse {
    person: String,
    date: String,
    hour: String,
}

fn extract_value(text: &str, key: &str) -> Option<String> {
    text.split_once(key).map(|(_, rest)| {
        rest.lines()
            .skip_while(|l| l.trim().is_empty())
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdfium = Pdfium::new(Pdfium::bind_to_library("./lib/libpdfium.so")?);

    let document = pdfium.load_pdf_from_file("ponto.pdf", None)?;
    let mut out = String::new();

    document.pages().iter().for_each(|page| {
        let text = page.text().expect("REASON").all();
        out.push_str(&text);
        out.push_str("\n--- Fim da página ---\n");
    });

    let person = extract_value(&out, "PESSOA:").unwrap_or_default();
    let mark = extract_value(&out, "MARCAÇÃO:").unwrap_or_default();

    let parts: Vec<&str> = mark.split_whitespace().collect();

    let date = parts.get(0).unwrap_or(&"").to_string();
    let hour = parts.get(1).unwrap_or(&"").to_string();

    let response = JSONResponse { person, date, hour };

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
