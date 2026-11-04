#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Token {
    lex: Lexeme,
    line: usize,
    column: usize,
    width: usize
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

fn tok_to_lexeme(token: String) -> Lexeme {
    let keywords: Vec<&str> = vec![
        "using",
        "namespace",
        "class",
        "inherits",
        "struct",
        "enum",
        "func",
        "var",
        "const",
        "public",
        "private",
        "return",
        "if",
        "else",
        "for",
        "while",
        "run",
        "jump",
        "true",
        "false",
    ];

    let operators: Vec<&str> = vec![
        ">>",
        "<<",
        "+",
        "-",
        "*",
        "/",
        "%",
        "^",
        "&",
        "|",
        "==",
        "!=",
        "&&",
        "||",

        "=",
        ">>=",
        "<<=",
        "+=",
        "-=",
        "*=",
        "/=",
        "%=",
        "^=",
        "&=",
        "|=",
    ];
    
    if keywords.contains(&token.as_str()) {
        Lexeme::KeyW(token)
    }
    else if operators.contains(&token.as_str()) {
        Lexeme::Op(token)
    }
    else if token.as_bytes()[0] == b'"' {
        Lexeme::StringLit(token)
    }
    else if token.as_bytes()[0] == b'\'' {
        Lexeme::Char(token.as_bytes()[0] as char)
    }
    else if token.parse::<i64>().is_ok() {
        Lexeme::Int(token.parse::<i64>().unwrap())
    }
    else if token.parse::<f64>().is_ok() {
        Lexeme::Float(token.parse::<f64>().unwrap())
    }
    else if token == "true" || token == "false" {
        Lexeme::Bool(token == "true")
    }
    else {
        match token.as_bytes()[0] as char {
            ';' => Lexeme::Semicolon,
            ':' => Lexeme::Colon,
            ',' => Lexeme::Comma,
            '.' => Lexeme::Dot,

            '(' => Lexeme::LParen,
            ')' => Lexeme::RParen,
            '[' => Lexeme::LSquare,
            ']' => Lexeme::RSquare,
            '{' => Lexeme::LBrace,
            '}' => Lexeme::RBrace,
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

    let delim_cont = ['&', '|', '>', '<'];

    let equals_cont = [
        '+', '-', '*', '%', '<', '>', '=', '!'
    ];
    let mut tok_str = String::new();
    let mut chars = code.char_indices().peekable();
    let mut pos = 0;

    while let Some(c) = chars.next() {
        pos = c.0 + c.1.len_utf8();

        if c.1.is_ascii_whitespace() {
            if tok_str.is_empty() {
                continue;
            } else {
                break;
            }
        }
        // STRING LITERAL
        if c.1 == '"' {
            tok_str.push(c.1);

            while let Some(next) = chars.next() {
                tok_str.push(next.1);

                // Handle escape sequences
                if next.1 == '\\' {
                    if let Some(escaped) = chars.next() {
                        tok_str.push(escaped.1);
                    }
                    continue;
                }

                // End of string
                if next.1 == '"' {
                    break;
                }
            }

            break;
        }

        // CHARACTER LITERAL
        if c.1 == '\'' {
            tok_str.push(c.1);

            while let Some(next) = chars.next() {
                tok_str.push(next.1);

                // Handle escape sequences
                if next.1 == '\\' {
                    if let Some(escaped) = chars.next() {
                        tok_str.push(escaped.1);
                        pos = escaped.0 + escaped.1.len_utf8();
                    }
                    continue;
                }

                // End of character
                if next.1 == '\'' {
                    break;
                }
            }

            break;
        }

        // NORMAL CHARACTERS
        if !delims.contains(&c.1) || (c.1 == '.' && tok_str.chars().all(|x| x.is_ascii_digit())) {
            tok_str.push(c.1);
            continue;
        }

        // FIRST DELIMITER
        if tok_str.is_empty() {
            tok_str.push(c.1);

            // Determine which characters can follow this operator
            let continuations: &[char] = if c.1 == '=' {
                &equals_cont
            } else if delim_cont.contains(&c.1) {
                &delim_cont
            } else {
                &[]
            };

            // Consume additional operator characters
            while let Some(&next) = chars.peek() {
                if continuations.contains(&next.1) {
                    let next = chars.next().unwrap();
                    tok_str.push(next.1);
                    pos = next.0 + next.1.len_utf8();
                } else {
                    break;
                }
            }
        }

        break;
    }

    let tok_len = tok_str.len();
    let tok: Token = Token {
        lex: tok_to_lexeme(tok_str),
        line,
        column,
        width: tok_len
    };

    let remaining = code[pos..].to_string();
    (remaining, tok)
}

pub fn tokenize(code: String) -> Vec<Token> {
    // Clean code of comments and empty line endings
    let clean = clean_code(code);
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut column = 1;

    let mut remaining: String = clean.clone();
    while !remaining.is_empty() {
        let old_remaining = remaining.clone();
        let (new_remaining, token) = add_token(old_remaining, line, column);
        
        let consumed_len = remaining.len() - new_remaining.len();
        if consumed_len == 0 {
            panic!("Tokenizer got stuck at line {}, column {}", line, column);
        }
        let consumed = &remaining[..consumed_len];
        
        for c in consumed.chars() {
            if c == '\n' {
                line += 1;
                column = 0;
            }
            column += 1;
        }

        remaining = new_remaining.clone();

        tokens.push(token);
    }

    tokens
}
