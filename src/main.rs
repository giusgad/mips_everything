use clap::Parser;
use mips_parser::MipsCompiler;
use std::{borrow::Borrow, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(required(true))]
    files: Vec<PathBuf>,
}

#[derive(thiserror::Error)]
enum MipsError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
impl std::fmt::Debug for MipsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

fn main() -> Result<(), MipsError> {
    let args = Args::parse();
    for path in args.files {
        if !path.try_exists()? {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File `{}` not found.", path.to_string_lossy()),
            )
            .into());
        }
        if !path.is_file() || path.extension().is_some_and(|e| e != "asm") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("`{}` is not an assembly file.", path.to_string_lossy()),
            )
            .into());
        }
        let file_content = std::fs::read_to_string(path.clone())?;
        let res = MipsCompiler::new(file_content.clone()).compile();
        match res {
            Ok(_) => {}
            Err(err) => {
                // the error is displayed with ariadne
                let a = path.file_name().unwrap().to_string_lossy();
                let file_name: &str = a.borrow();
                err.display_formatted(file_name.to_owned(), &file_content)?;
            }
        };
    }
    Ok(())
}
