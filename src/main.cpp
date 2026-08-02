#include <fstream>
#include <iostream>
#include <vector>
#include <cctype>


enum TokenType {
    SEMICOLON,
    IDENTIFIER,
    CHAR,
    NUMBER,
    OPERATOR,
    COLON,

    DOUBLE_QUOTATION,
    SINGLE_QUOTATION,
    
    OPEN_BRACKET,
    CLOSE_BRACKET,

    OPEN_SQUARE_BRACKET,
    CLOSE_SQUARE_BRACKET,

    OPEN_SQUIGGLY_BRAKET,
    CLOSE_SQUIGGLY_BRACKET,
};

struct Line {
    std::string contents;
    int line;
};

struct Token {
    std::string contents;
    int line;
    TokenType token_type;
};


int read_file(const std::string& path, std::string& data) {
    std::ifstream f(path, std::ios::binary | std::ios::ate);

    if (!f.is_open()) {
        std::cerr << "Unable to read " << path << std::endl;
        return 1;
    }

    std::streamsize size = f.tellg();
    f.seekg(0);

    data.resize(size);

    f.read(data.data(), size);

    f.close();
    return 0;
}

std::vector<Line> split_lines(std::string code) {
    std::vector<Line> split_code;
    split_code.resize(1);
    
    int line = 0;
    
    for (char c: code) {
        split_code[line].line = line + 1;

        if (c != '\n') {
            split_code[line].contents += c;
        } else {
            split_code.resize(++line + 1);
        }
    }
    
    std::vector<Line> split_code_clean;

    for (Line l: split_code) {
        if ((l.line != 0) && (l.contents != "")) {
            split_code_clean.push_back(l);
        }
    }

    return split_code_clean;
}

std::vector<Line> clean_code(std::vector<Line> code) {
    std::vector<Line> cleaned;

    for (Line& line : code) {
        std::string result;
        bool previous_space = false;

        for (char c : line.contents) {
            if (c == '#')
                break;

            if (std::isspace(static_cast<unsigned char>(c))) {
                if (!result.empty() && !previous_space) {
                    result += ' ';
                    previous_space = true;
                }
            }
            else {
                result += c;
                previous_space = false;
            }
        }

        if (!result.empty() && result.back() == ' ')
            result.pop_back();

        if (!result.empty()) {
            line.contents = std::move(result);
            cleaned.push_back(std::move(line));
        }
    }

    return cleaned;
}

std::vector<Line> tokenize(const std::string& code) {
    std::vector<Line> lines = clean_code(split_lines(code));

    std::vector<Line> raw_tokens;

    for (const Line& line : lines) {
        std::string token;
        bool in_string = false;

        auto push_token = [&]() {
            if (!token.empty()) {
                raw_tokens.push_back({
                    .contents = token,
                    .line = line.line,
                });
                token.clear();
            }
        };

        for (char c : line.contents) {
            if (in_string) {
                token += c;

                if (c == '"') {
                    push_token();
                    in_string = false;
                }

                continue;
            }

            if (c == '"') {
                push_token();
                token += c;
                in_string = true;
                continue;
            }

            if (std::isspace(static_cast<unsigned char>(c))) {
                push_token();
                continue;
            }

            switch (c) {
                case '(':
                case ')':
                case '{':
                case '}':
                case '[':
                case ']':
                case ',':
                case ';':
                case ':':
                case '+':
                case '-':
                case '*':
                case '/':
                case '=':
                    push_token();
                    raw_tokens.push_back({
                        .contents = std::string(1, c),
                        .line = line.line,
                    });
                    break;

                default:
                    token += c;
            }
        }

        push_token();
    }

    return raw_tokens;
}

int main(int argc, char** argv) {
    if (argc == 1) {
        std::cerr << "No arguments" << std::endl;
        return 1;
    }

    std::string data;
    int status = read_file(argv[1], data);

    std::vector<Line> tokens = tokenize(data);

    for (Line tok: tokens)
        std::cout << tok.line << " " << tok.contents << std::endl;

    return status;
}

