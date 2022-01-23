# SFA (Rust)

Single File Assets is a file storage format for images. The packed images are not guarenteed
to be of same format because the format while saving is changed to png which is lossless.
So expectation of jpeg image being extracted from a sfa file being same does not work.

## Example

You can use it to make a spritesheet. For example with the following images with us:

* sprite_1.png
* sprite_2.png
* sprite_3.png

we can convert these multiple files into one using the sfa command line utility which
can be downloaded from our release page or by simply doing as follows:

```
cargo install sfa
``` 

and then the utility can be used as follows

```
sfa pack sprite_1.png sprite_2.png sprite_3.png -o sprite.sfa
```

Now the above generated sfa file can be used in our program

```rust
use sfa::decode;
use std::collections::HashMap;

fn main() {
    // Load the file with images in memory (You can skip the type annotations)
    let sprite: HashMap<String, image::DynamicImage> = decode("sprite.sfa").expect("Unexpected error occured");

    // Now separate images can be accessed with there original name
    let frame_1 = &sprite["sprite_1.png"];
}
```
