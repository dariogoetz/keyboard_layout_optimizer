"""
This script checks your layout-config and removes all impossible ngrams from row specified ngrams-directory.
"""

import yaml
import os
import shutil


def load_yaml_from_file(yaml_file):
    with open(yaml_file, "r") as file:
        yaml_code = file.read()
        yaml_data = yaml.safe_load(yaml_code)
    return yaml_data


def filter_ngrams(layout_chars, ngram_dir, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    count = 0

    for i, filename in enumerate(os.listdir(ngram_dir)):
        if filename.endswith(".txt"):
            input_filepath = os.path.join(ngram_dir, filename)
            output_filepath = os.path.join(output_dir, filename)
            with open(input_filepath, "r") as input_file, open(
                output_filepath, "w"
            ) as output_file:
                print("Processing", input_filepath, "→", output_filepath)
                for line in input_file:
                    # Split line into frequency and ngram
                    frequency, ngram = line.split(" ", 1)
                    ngram = ngram[:-1]
                    # ngram = ngram.rstrip()  # Remove trailing whitespace and newline

                    valid_ngram = True
                    for char in ngram:
                        if char not in layout_chars:
                            valid_ngram = False
                            break

                    """ if (not valid_ngram) and (count < 100) and (filename[0] == "1"):
                        count += 1
                        print(count, frequency, ngram, valid_ngram) """

                    if valid_ngram:
                        output_file.write(line)


# Example usage
yaml_file = "config/keyboard/my_keyboard_config.yml"  # Specify the keyboard-config here
ignore_in_layout = "☒■⇩⇘⇧⇗♕⇇↜⇉↝♛"  # ♔
ngram_dir = "ngrams/made_up_dir"
output_dir = "ngrams/made_up_dir_reduced"

yaml_data = load_yaml_from_file(yaml_file)
layout_chars = set()
print("\nCharacters in Layout:")
for row in yaml_data["base_layout"]["keys"]:
    for key in row:
        print(key)
        for c in key:
            layout_chars.add(c)

for c in ignore_in_layout:
    layout_chars.remove(c)

filter_ngrams(layout_chars, ngram_dir, output_dir)
