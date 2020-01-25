# Binary to BMP image converter

binary-to-bmp-converter is a utility that generates bmp images based on any file given as input. Basically reads the raw contents of the provided file and creates pixels by taking 3 by 3 bytes and writes the bmp image.

This app is my first learn by doing project in rust as serves also as a simple file handling example in rust.

## Build

```
cargo build
```

## Usage

```
binary-to-bmp-converter foo.bin
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
