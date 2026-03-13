import sys
import tarfile


def main():
    args = sys.argv[1:]
    if len(args) != 2:
        print("usage: python extract.py <ARCHIVE_PATH> <TARGET_PATH>")
        sys.exit(1)

    archive, target = args

    with tarfile.open(archive, "r:gz") as tf:
        tf.extractall(target)

    print("extracted")


if __name__ == "__main__":
    main()
