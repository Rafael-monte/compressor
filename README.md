# Compressor
### Rust app to compress text files in a simple way

## Usual usage:
```bash
# To compress file
./compressor -c example_text_file.txt

# To revert compression
./compressor -d compressed_file.rco decomp_key.rcok
```

## Flags
```bash
# Show info about project
-i

# Compress the file
-c [input_file]

# Reverts compression
-d [compressed_file] [compression_key]
```

## How to use it?
1. Go to the [release page](https://github.com/Rafael-monte/compressor/releases/tag/v.0.1.0h) and install the binary file;
2. Run the following command in your shell:
```bash
chmod +x compressor
```
3. And it's done! Just note that the `compressed.rco` and `decomp-key.rcok` files are generated in the same directory that the command was run 
