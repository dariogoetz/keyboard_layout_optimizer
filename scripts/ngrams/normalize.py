import argparse
import sys
import os


def main(ngrams_directory):
    filenames = [
        os.path.join(ngrams_directory, "1-grams.txt"),
        os.path.join(ngrams_directory, "2-grams.txt"),
        os.path.join(ngrams_directory, "3-grams.txt")
    ]

    for filename in filenames:
        fTot = 0
        f = []
        l = []
        with open(filename) as ngrams:
            i = 0
            for ngram in ngrams:
                freqStr, letters = ngram.split(" ", 1)
                freq = float(freqStr)

                f.append(freq)
                l.append(letters)
                fTot += freq

        with open(filename, "w") as ngrams:
            for freq, ngram in zip(f, l):
                ngrams.write(str(100 * freq / fTot) + " " + ngram)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Normalize n-gram frequencies in a directory of n-gram files."
                                                 "Normalization converts absolute frequencies into percentages of "
                                                 "how often an n-gram occurs within the corpus.")
    parser.add_argument("ngrams_directory", help="Path to the directory containing the n-gram files.")
    args = parser.parse_args()

    if not os.path.isdir(args.ngrams_directory):
        print("Error: Invalid n-gram directory path. Please provide a valid directory path.", file=sys.stderr)
        sys.exit(1)

    main(args.ngrams_directory)
