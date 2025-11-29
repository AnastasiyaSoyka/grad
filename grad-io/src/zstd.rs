use thiserror::{Error};
use tracing::{instrument, trace};

#[derive(Debug, Error)]
pub enum DecompressError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error)
}

#[instrument(skip_all)]
pub fn decompress_lines(data: &[u8]) -> Result<Vec<String>, DecompressError> {
    let buffer = zstd::stream::decode_all(data)?;

    trace!(size = buffer.len(), packed_size = data.len(), ratio = (data.len() as f64) / (buffer.len() as f64));

    let mut lines: Vec<String> = String::from_utf8(buffer)?
        .lines()
        .filter(|value | ! value.is_empty())
        .map(String::from)
        .collect();

    lines.sort();
    lines.dedup();

    trace!(words = lines.len());

    Ok(lines)
}
