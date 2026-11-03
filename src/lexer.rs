enum Lexeme {
    KeyW(String),
    Iden(String),
    Op(String),
    Int(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    StringLit(String),

    Semicolon,
    Colon,
    Comma,
    Dot,

    LBrace,
    RBrace,
    LParen,
    RParen,
    LSquare,
    RSquare,

    Eof,
}

struct Token {
    lex: Lexeme,
    line: usize,
    column: usize
}

fn clean_code(code: String) -> String {
    let mut commentless = String::new();
    let mut in_comment = false;

    let mut string_delimiter: Option<char> = None;
    let mut escaped = false;

    // Get rid of comments
    for ch in code.chars() {
        // Handle escape sequences
        if string_delimiter.is_some() && escaped {
            escaped = false;
            commentless.push(ch);
            continue;
        }

        if string_delimiter.is_some() && ch == '\\' {
            escaped = true;
            commentless.push(ch);
            continue;
        }

        // Handle newlines
        if ch == '\n' && string_delimiter.is_none() {
            in_comment = false;
            commentless.push(ch);
            continue;
        }

        // Handle string and character delimiters
        if !in_comment {
            if let Some(delimiter) = string_delimiter {
                if ch == delimiter {
                    string_delimiter = None;
                }
            } else if ch == '\'' || ch == '"' {
                string_delimiter = Some(ch);
            }
        }

        // Handle comments
        if ch == '#' && string_delimiter.is_none() {
            in_comment = true;
            continue;
        }

        // Push remaining characters
        if !in_comment {
            commentless.push(ch);
        }
    }

    // Remove trailing whitespace
    commentless
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn tokenize(code: String) -> Vec<Token> {
    // Clean code of comments and empty line endings
    let clean = clean_code(code);

    Vec::new()
}
