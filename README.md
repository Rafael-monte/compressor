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
1. Go to the [release page](https://github.com/Rafael-monte/compressor/releases/tag/v.0.1.1h) and install the binary file;
2. Run the following command in your shell:
```bash
chmod +x compressor
```
3. And it's done! Just note that the `compressed.rco` and `decomp-key.rcok` files are generated in the same directory that the command was run 


## Some comparitions

Considering the following files:
- light-lorem.txt, a small text file
- medium-lorem.txt, a medium-size text file
- heavy-lorem.txt, a large size text file

There are some results after compression and revertion:

| File Name         | Original Size | Number of Paragraphs | Compressed Size | Key File Size | After Decompression |
|-------------------|---------------|----------------------|-----------------|---------------|---------------------|
| light-lorem.txt   | 446B          | 5                    | 200B            | 1012B         | 447B                |
| medium-lorem.txt  | 77Kb          | 100                  | 44Kb            | 8.7Kb         | 77Kb               |
| heavy-lorem.txt   | 746Kb         | 1000                 | 425Kb           | 8.8Kb         | 747Kb              |
