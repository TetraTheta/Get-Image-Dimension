# Get-Image-Dimension

Prints the dimensions of an image in 'width x height' format.

Error messages will be printed to STDERR. Only valid output(`width x height`) will be printed to STDOUT.

```plaintext
>get-image-dimension --help
Prints the dimensions of an image in 'width x height' format.

Usage: get-image-dimension [OPTIONS] [FILE]

Arguments:
  [FILE]  Target image

Options:
  -w, --width    Output only the width of the image
  -h, --height   Output only the height of the image
  -h, --help     Print help
  -V, --version  Print version

>get-image-dimension --version
get-image-dimension 1.0.0

>get-image-dimension image.png
1920 x 1080

>get-image-dimension image.png -w
1920

>get-image-dimension image.png -h
1080

>get-image-dimension path/to/invalid/file.png
ERROR Could not open the image file or the file is not a valid image.

>get-image-dimension
ERROR No file path is provided.

>get-image-dimension image.png -w -w
error: the argument '--width' cannot be used multiple times

Usage: get-image-dimension [OPTIONS] [FILE]

For more information, try '--help'.
```
