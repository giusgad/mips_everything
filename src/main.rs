use clap::Parser;
use mips_parser::{errors::CompileError, MipsCompiler};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(required(true))]
    files: Vec<PathBuf>,
}

#[derive(thiserror::Error)]
enum MipsError {
    #[error("Cli error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Compile(#[from] CompileError),
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
                let file_name = path.file_name().unwrap().to_string_lossy();
                //TODO: filename
                err.display_formatted("test.asm", &file_content)?;
            }
        };
    }
    Ok(())
}
