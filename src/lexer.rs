use std::fmt;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Lexeme {
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

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexeme::KeyW(v) => write!(f, "Keyword({})", v),
            Lexeme::Iden(v) => write!(f, "Identifier({})", v),
            Lexeme::Op(v) => write!(f, "Operator({})", v),
            Lexeme::Int(v) => write!(f, "Integer({})", v),
            Lexeme::Float(v) => write!(f, "Float({:?})", v),
            Lexeme::Char(v) => write!(f, "Char('{}')", v),
            Lexeme::Bool(v) => write!(f, "Bool({})", v),
            Lexeme::StringLit(v) => write!(f, "String(\"{}\")", v),

            Lexeme::Semicolon => write!(f, "Semicolon"),
            Lexeme::Colon => write!(f, "Colon"),
            Lexeme::Comma => write!(f, "Comma"),
            Lexeme::Dot => write!(f, "Dot"),

            Lexeme::LBrace => write!(f, "LBrace"),
            Lexeme::RBrace => write!(f, "RBrace"),
            Lexeme::LParen => write!(f, "LParen"),
            Lexeme::RParen => write!(f, "RParen"),
            Lexeme::LSquare => write!(f, "LSquare"),
            Lexeme::RSquare => write!(f, "RSquare"),

            Lexeme::Eof => write!(f, "Eof"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Token {
    pub lex: Lexeme,
    pub line: usize,
    pub column: usize,
    pub width: usize
}

pub fn clean_code(code: String) -> String {
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

fn tok_to_lexeme(token: String) -> Lexeme {
    let keywords = [
        "using", "namespace", "class", "inherits", "struct",
        "enum", "func", "var", "const", "public", "private",
        "return", "if", "else", "for", "while", "run", "jump"
    ];

    let operators = [
        ">>", "<<", ">>>",
        "+", "-", "*", "/", "%", "^", "&", "|",
        "==", "!=", "&&", "||",
        "=", ">>=", "<<=", ">>>=",
        "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=",
        ">", "<", ">=", "<=", "!", "~"
    ];

    if token.is_empty() {
        panic!("Attempted to convert an empty token");
    }

    if keywords.contains(&token.as_str()) {
        Lexeme::KeyW(token)
    }
    else if token == "true" || token == "false" {
        Lexeme::Bool(token == "true")
    }
    else if operators.contains(&token.as_str()) {
        Lexeme::Op(token)
    }
    else if token.starts_with('"') {
        Lexeme::StringLit(token)
    }
    else if token.starts_with('\'') {
        let value = &token[1..token.len() - 1];
        let ch = match value {
            "\\n" => '\n',
            "\\t" => '\t',
            "\\r" => '\r',
            "\\0" => '\0',
            "\\\\" => '\\',
            "\\'" => '\'',
            _ => value.chars().next().unwrap()
        };

        Lexeme::Char(ch)
    }
    else if let Ok(value) = token.parse::<i64>() {
        Lexeme::Int(value)
    }
    else if let Ok(value) = token.parse::<f64>() {
        Lexeme::Float(value)
    }
    else {
        match token.as_str() {
            ";" => Lexeme::Semicolon,
            ":" => Lexeme::Colon,
            "," => Lexeme::Comma,
            "." => Lexeme::Dot,

            "(" => Lexeme::LParen,
            ")" => Lexeme::RParen,
            "[" => Lexeme::LSquare,
            "]" => Lexeme::RSquare,
            "{" => Lexeme::LBrace,
            "}" => Lexeme::RBrace,

            _ => Lexeme::Iden(token)
        }
    }
}

fn add_token(code: String, line: usize, column: usize) -> (String, Token) {
    let delims = [
        ' ', ',', '.', ';', ':', '-', '=', '+', '*',
        '>', '<', '/', '%', '!', '&', '|', '^',
        '[', ']', '(', ')', '{', '}'
    ];

    let operators = [
        ">>>=", ">>=", "<<=",
        "==", "!=", ">=", "<=",
        "&&", "||", ">>", "<<",
        "+=", "-=", "*=", "/=", "%=",
        "^=", "&=", "|=",
        "++", "--", "->", "::",
        "+", "-", "*", "/", "%", "^",
        "&", "|", "=", ">", "<", "!", "~"
    ];

    let mut tok_str = String::new();
    let mut chars = code.char_indices().peekable();
    let mut pos = 0;

    while let Some(c) = chars.next() {
        // WHITESPACE
        if c.1.is_ascii_whitespace() {
            if tok_str.is_empty() {
                pos = c.0 + c.1.len_utf8();
                continue;
            } else {
                break;
            }
        }

        // STRING LITERAL
        if c.1 == '"' {
            tok_str.push(c.1);
            pos = c.0 + c.1.len_utf8();

            while let Some(next) = chars.next() {
                tok_str.push(next.1);
                pos = next.0 + next.1.len_utf8();

                if next.1 == '\\' {
                    if let Some(escaped) = chars.next() {
                        tok_str.push(escaped.1);
                        pos = escaped.0 + escaped.1.len_utf8();
                    }
                    continue;
                }

                if next.1 == '"' {
                    break;
                }
            }

            break;
        }

        // CHARACTER LITERAL
        if c.1 == '\'' {
            tok_str.push(c.1);
            pos = c.0 + c.1.len_utf8();

            while let Some(next) = chars.next() {
                tok_str.push(next.1);
                pos = next.0 + next.1.len_utf8();

                if next.1 == '\\' {
                    if let Some(escaped) = chars.next() {
                        tok_str.push(escaped.1);
                        pos = escaped.0 + escaped.1.len_utf8();
                    }
                    continue;
                }

                if next.1 == '\'' {
                    break;
                }
            }

            break;
        }

        // NORMAL CHARACTERS
        if !delims.contains(&c.1)
            || (c.1 == '.' && !tok_str.is_empty()
                && tok_str.chars().all(|x| x.is_ascii_digit()))
        {
            tok_str.push(c.1);
            pos = c.0 + c.1.len_utf8();
            continue;
        }

        // FIRST DELIMITER
        if tok_str.is_empty() {
            let mut matched = String::new();

            for op in operators {
                if code.starts_with(op) && op.len() > matched.len() {
                    matched = op.to_string();
                }
            }

            if !matched.is_empty() {
                tok_str = matched;
                pos = c.0 + tok_str.len();

                // Consume remaining characters of the operator
                for _ in 1..tok_str.chars().count() {
                    chars.next();
                }
            } else {
                tok_str.push(c.1);
                pos = c.0 + c.1.len_utf8();
            }
        }

        break;
    }

    let tok_len = tok_str.len();

    let tok = Token {
        lex: tok_to_lexeme(tok_str),
        line,
        column,
        width: tok_len
    };

    let remaining = code[pos..].to_string();

    (remaining, tok)
}

pub fn tokenize(code: String) -> Vec<Token> {
    let clean = clean_code(code);

    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut column = 1;

    let mut remaining = clean;

    while !remaining.is_empty() {
        // Consume leading whitespace
        let whitespace_len = remaining
            .char_indices()
            .take_while(|(_, c)| c.is_ascii_whitespace())
            .map(|(i, c)| i + c.len_utf8())
            .last()
            .unwrap_or(0);

        if whitespace_len > 0 {
            let whitespace = &remaining[..whitespace_len];

            for c in whitespace.chars() {
                if c == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }

            remaining = remaining[whitespace_len..].to_string();
        }

        if remaining.is_empty() {
            break;
        }

        let (new_remaining, token) =
            add_token(remaining.clone(), line, column);

        let consumed_len = remaining.len() - new_remaining.len();
        if consumed_len == 0 {
            panic!("Tokenizer got stuck at line {}, column {}", line, column);
        }

        let consumed = &remaining[..consumed_len];

        for c in consumed.chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        remaining = new_remaining;
        tokens.push(token);
    }

    tokens
}
