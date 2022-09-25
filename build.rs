use std::{
    env::{self, VarError}, path::Path, fs::File, io::{BufWriter, Write}
};

#[derive(Debug)]
enum Error {
    VarError(VarError),
    IoError(std::io::Error)
}

impl From<VarError> for Error {
    fn from(error: VarError) -> Self {
        Self::VarError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

fn main() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("app.css");
    let mut f = BufWriter::new(File::create(&dest_path)?);

    let compiled_css = grass::from_string(include_str!("assets/scss/app.scss").to_owned(), &grass::Options::default());
    write!(f, "{}", compiled_css.unwrap())?;

    Ok(())
}