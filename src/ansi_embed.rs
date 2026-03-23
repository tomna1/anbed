pub fn embed_ansi(text: &str) -> String {
    text.replace("\\x1b", "\x1b")
        .replace("\\033", "\x1b")
        .replace("\\e", "\x1b")
}
