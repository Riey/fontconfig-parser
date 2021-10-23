use fontconfig_parser::{parse_document_from_str, Document, Result};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn visit_document(doc: &Document, doc_path: PathBuf, dirs: &mut HashSet<PathBuf>) -> Result<()> {
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
                let doc_path_name = doc_path.file_name().unwrap().to_str().unwrap();
                if entry
                    .file_type()
                    .map_or(false, |t| t.is_file() || t.is_symlink())
                    && matches!(
                        doc_path_name.as_bytes(),
                        [b'0'..=b'9', .., b'.', b'c', b'o', b'n', b'f']
                    )
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
            let doc_conf = std::fs::read_to_string(&doc_path)?;
            let doc = parse_document_from_str(&doc_conf)?;
            visit_document(&doc, doc_path, dirs)?;
        }
    }

    Ok(())
}

fn visit_dir(dir: PathBuf, fonts: &mut Vec<PathBuf>) -> Result<()> {
    let dir = std::fs::read_dir(dir)?;

    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(ty) = entry.file_type() {
                if ty.is_dir() {
                    visit_dir(entry.path(), fonts).ok();
                } else if ty.is_file() || ty.is_symlink() {
                    fonts.push(entry.path());
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let root_path = PathBuf::from("/etc/fonts/fonts.conf");
    let root_config = std::fs::read_to_string(&root_path)?;
    let root = parse_document_from_str(&root_config)?;
    let mut dirs = HashSet::new();

    visit_document(&root, root_path, &mut dirs)?;

    println!("dirs: {:#?}", dirs);

    let mut fonts = Vec::new();

    for dir in dirs {
        visit_dir(dir, &mut fonts).ok();
    }

    println!("Find all {} fonts!", fonts.len());

    println!("fonts: {:#?}", fonts);

    Ok(())
}
