use fontconfig_parser::{quick_xml::Reader, Document, DocumentReader, Result};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn visit_document(
    doc: &Document,
    doc_path: PathBuf,
    reader: &mut DocumentReader,
    dirs: &mut HashSet<String>,
) -> Result<()> {
    dirs.extend(doc.dirs.iter().map(|d| d.path.clone()));

    for include in doc.includes.iter() {
        let include_path = include.calculate_path(&doc_path);

        println!("Include: {}", include_path.display());

        if !include_path.exists() {
            if !include.ignore_missing {
                eprintln!("Include path {} is missing!", include_path.display());
            }
            continue;
        }

        for entry in fs::read_dir(include_path)? {
            if let Ok(entry) = entry {
                if entry
                    .file_type()
                    .map_or(false, |t| t.is_file() || t.is_symlink())
                {
                    let doc_path = entry.path();
                    println!("Read: {}", doc_path.display());
                    let doc = reader.read_document(&mut Reader::from_file(&doc_path)?)?;
                    visit_document(&doc, doc_path, reader, dirs)?;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut reader = DocumentReader::new();
    let mut dirs = HashSet::new();

    let root_path = PathBuf::from("/etc/fonts/fonts.conf");
    let root = reader.read_document(&mut Reader::from_file(&root_path)?)?;

    visit_document(&root, root_path, &mut reader, &mut dirs)?;

    println!("{:#?}", dirs);

    Ok(())
}
