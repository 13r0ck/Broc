# Static site generator

This is an example of how you might build a static site generator using Broc.
It searches for Markdown (`.md`) files in the `input` directory, inserts them
into a HTML template defined in Broc, and writes the result into the
corresponding file path in the `output` directory.

To run, `cd` into this directory and run this in your terminal:

If `broc` is on your PATH:
```bash
broc run static-site.broc -- input/ output/
```

If not, and you're building Broc from source:
```
cargo run -- static-site.broc -- input/ output/
```

The example in the `input` directory is a copy of the 2004 website
by John Gruber, introducing the Markdown format.
https://daringfireball.net/projects/markdown/
