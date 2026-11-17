use agentic_wiki::domain::{PassageAnchor, PassagePatch, RichDocument, map_anchor};
use serde::Deserialize;
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Input {
    document: RichDocument,
    patch: PassagePatch,
    #[serde(default)]
    anchors: Vec<PassageAnchor>,
}
fn main() {
    let mut raw = String::new();
    io::stdin()
        .take(3_000_001)
        .read_to_string(&mut raw)
        .unwrap();
    let result = (|| -> Result<serde_json::Value, String> {
        if raw.len() > 3_000_000 {
            return Err("input too large".into());
        }
        let input: Input = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        let document = input
            .document
            .replace_passage(&input.patch)
            .map_err(|e| e.to_string())?;
        let anchors: Vec<_> = input
            .anchors
            .iter()
            .map(|a| map_anchor(a, &input.patch))
            .collect();
        Ok(serde_json::json!({"document":document,"anchors":anchors}))
    })();
    match result {
        Ok(v) => println!("{v}"),
        Err(e) => {
            println!("{}", serde_json::json!({"error":e}));
            std::process::exit(2)
        }
    }
}
