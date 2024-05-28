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
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Compilation error: {0}")]
    Compile(#[from] CompileError),
}
impl std::fmt::Debug for MipsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(arg) => write!(f, "{arg}"),
            Self::Compile(arg) => write!(f, "{arg}"),
        }
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
        MipsCompiler::new(std::fs::read_to_string(path)?).compile()?
    }
    Ok(())
}
