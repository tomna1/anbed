use crate::AnbedError;

pub fn embed_ansii(text: &str) -> Result<String, AnbedError> {
    Ok(text.replace("\\x1b", "\x1b").replace("\\033", "\x1b"))
}
