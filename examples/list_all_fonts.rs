use fontconfig_parser::{quick_xml::Reader, Document, DocumentReader, Result};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn visit_document(
    doc: &Document,
    name_regex: &regex::Regex,
    doc_path: PathBuf,
    reader: &mut DocumentReader,
    dirs: &mut HashSet<PathBuf>,
) -> Result<()> {
    dirs.extend(doc.dirs.iter().map(|d| d.calculate_path(&doc_path)));

    for include in doc.includes.iter() {
        let include_path = include.calculate_path(&doc_path);

        println!("Include: {}", include_path.display());

        if !include_path.exists() {
            if !include.ignore_missing {
                eprintln!("Include path {} is missing!", include_path.display());
            }
            continue;
        }

        let dir = fs::read_dir(include_path)?;

        let mut paths = dir
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let doc_path = entry.path();
                if entry
                    .file_type()
                    .map_or(false, |t| t.is_file() || t.is_symlink())
                    && name_regex.is_match(doc_path.as_os_str().to_str().unwrap())
                {
                    Some(doc_path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        paths.sort();

        for doc_path in paths {
            println!("Read: {}", doc_path.display());
            let doc = reader.read_document(&mut Reader::from_file(&doc_path)?)?;
            visit_document(&doc, name_regex, doc_path, reader, dirs)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut reader = DocumentReader::new();
    let mut dirs = HashSet::new();
    let name_regex = regex::Regex::new(r#"[0-9].+\.conf"#).unwrap();

    let root_path = PathBuf::from("/etc/fonts/fonts.conf");
    let root = reader.read_document(&mut Reader::from_file(&root_path)?)?;

    visit_document(&root, &name_regex, root_path, &mut reader, &mut dirs)?;

    println!("{:#?}", dirs);

    Ok(())
}
