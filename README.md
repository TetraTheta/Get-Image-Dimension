# Get-Image-Dimension

Prints the dimensions of an image in 'width x height' format.

Error messages will be printed to STDERR. Only valid output(`width x height`) will be printed to STDOUT.

```plaintext
>get-image-dimension path/to/invalid/file.png
ERROR Could not open the image file or the file is not a valid image.

>get-image-dimension
ERROR No file path is provided.

>get-image-dimension --version
get-image-dimension 1.0.0

>get-image-dimension --help
Prints the dimensions of an image in 'width x height' format.

Usage: get-image-dimension [FILE]

Arguments:
  [FILE]

Options:
  -h, --help     Print help
  -V, --version  Print version

>get-image-dimension image.png
1920 x 1080
```
