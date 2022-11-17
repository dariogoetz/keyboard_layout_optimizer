import json
from pathlib import Path

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser("Parse OXEY's json file into ngrams")

    parser.add_argument("filename", type=Path, help="JSON file to parse")
    parser.add_argument(
        "out_dir",
        type=Path,
        help="Target directory (will be generated if not existing)",
    )

    args = parser.parse_args()

    with open(args.filename) as fp:
        data = json.load(fp)

    args.out_dir.mkdir(exist_ok=True)
    for (ngram_type, target_filename) in [
        ("characters", "1-grams.txt"),
        ("bigrams", "2-grams.txt"),
        ("trigrams", "3-grams.txt"),
    ]:
        with open(args.out_dir / target_filename, "w") as fp:
            print(f"Writing {len(data[ngram_type])} grams to {fp.name}")
            for gram, weight in data[ngram_type].items():
                fp.write(f"{weight} {gram}\n")
