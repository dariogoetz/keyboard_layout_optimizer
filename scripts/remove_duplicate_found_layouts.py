"""
This script removes all the duplicate layouts from your `solutions.txt`-file.
"""

def main():
    originalCount = 0
    uniqueLayouts = [] # A list is used instead of a set to preserve ordering.

    # Fill up [uniqueLayouts].
    with open("../solutions.txt") as layouts:
        for layout in layouts:
            originalCount += 1
            if layout not in uniqueLayouts:
                uniqueLayouts.append(layout)

    if originalCount == len(uniqueLayouts):
        print ("There are no duplicate Layouts.")
    else:
        # Write all unique layouts to the same file, replacing the old text.
        with open("../solutions.txt", "w") as layouts:
            for layout in uniqueLayouts:
                layouts.write(layout)

        # Display results
        print("Updated file!")
        print("Original count:", originalCount,)
        print("New count:     ", len(uniqueLayouts))

if __name__ == "__main__":
    main()
