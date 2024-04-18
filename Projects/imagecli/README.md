# Image resizing tool

https://learning.oreilly.com/library/view/practical-system-programming/9781800560963/B16405_04_Final_NM_ePUB.xhtml#_idParaDest-71

https://github.com/PacktPublishing/Practical-System-Programming-for-Rust-Developers/tree/master/Chapter04

Management Environment, Command Line parameter, and Time

Create a tool for bulk image resizing – tool that would look through a filesystem directory on a desktop or server, pull out all the image files (for instance, .png and .jpg), and resize all of them to predefined sizes (for example, small, medium, or large).

Crate project

```bash
cargo new imagecli
cd imagecli
```

Add dependencies

```toml
image = "0.23.7"
structopt = "0.3.15"
```
