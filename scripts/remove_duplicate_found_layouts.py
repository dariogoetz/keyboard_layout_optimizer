"""
This script removes all the duplicate layouts from a specified file.
"""

import argparse
import os


def remove_duplicates(filename):
    originalCount = 0
    uniqueLayouts = []  # A list is used instead of a set to preserve ordering.

    # Fill up [uniqueLayouts].
    with open(filename) as layouts:
        for layout in layouts:
            originalCount += 1
            if layout not in uniqueLayouts:
                uniqueLayouts.append(layout)

    if originalCount == len(uniqueLayouts):
        print("There are no duplicate Layouts.")
    else:
        # Write all unique layouts to the same file, replacing the old text.
        with open(filename, "w") as layouts:
            for layout in uniqueLayouts:
                layouts.write(layout)

        # Display results
        print("Updated file!")
        print("Original count:", originalCount)
        print("New count:     ", len(uniqueLayouts))


def main():
    parser = argparse.ArgumentParser(
        description="Remove duplicate layouts from a file."
    )  # Create an argument parser
    parser.add_argument(
        "filename", help="Name of the file to process"
    )  # Add the required unnamed command-line parameter for the filename
    args = parser.parse_args()  # Parse the command-line arguments

    # Check if the file exists
    if not os.path.exists(args.filename):
        print(f"Error: The file '{args.filename}' does not exist.")
        exit(1)

    # Call the function to remove duplicates
    remove_duplicates(args.filename)


if __name__ == "__main__":
    main()
