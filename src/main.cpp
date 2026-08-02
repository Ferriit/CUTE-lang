#include <fstream>
#include <iostream>

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

int main() {
    std::string data;
    int status = read_file("cute.toml", data);

    return 0;
}

