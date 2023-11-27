import os
import re
import itertools

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("infile", help="Corpus file to process")
    parser.add_argument("outfile", help="Result filename")

    args = parser.parse_args()

    # Check if input file exists
    if not os.path.exists(args.infile):
        print(f"Input file '{args.infile}' does not exist.")
        exit(1)
    # Check if output file exists
    if os.path.exists(args.outfile):
        print(
            f"Warning: Output file '{args.outfile}' already exists and will be overwritten."
        )

    # delete leading line numbers
    os.system(f"cut -f2 {args.infile} > {args.outfile}")
    # replace 4 out of 5 line breaks with spaces
    with open(args.outfile) as fp:
        s = fp.read()

    res = re.sub(
        "(\n)", lambda m, c=itertools.count(): m.group() if next(c) % 5 == 4 else " ", s
    )

    with open(args.outfile, "w") as fp:
        fp.write(res)
