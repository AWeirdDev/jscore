import sys


def main():
    args = sys.argv[1:]
    if len(args) != 1:
        print("usage: python extract.py <ARCHIVE_PATH>")
        exit(1)

    # crazy that this is from the standard library
    from zipfile import ZipFile

    with ZipFile(args[0], "r") as zr:
        zr.extractall()

    print("extracted")
