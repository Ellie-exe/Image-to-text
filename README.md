# Image-to-text
Converts images into ASCII characters.

## Usage
Commands are formated as follows: `cargo run some-image.png -o output-file.txt -c` where -o specifies the output file and -c specifies it should be in color. The color defaults to black and white meaning just pure text, were as color uses ANSI escape sequences to add color to the image. To be able to see colors you must use a terminal or text viewer that supports ANSI.
