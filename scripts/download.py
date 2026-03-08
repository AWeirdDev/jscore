import sys


def main():
    args = sys.argv[1:]
    if len(args) != 2:
        print("usage: python download.py <URL> <ARCHIVE_PATH>")
        exit(1)

    try:
        import requests
    except ImportError:
        from pip._internal.cli.main import main

        print("couldn't find requests for python; installing requests")
        main(["install", "requests"])
        import requests

    res = requests.get(args[0])
    with open(args[1], "wb") as f:
        f.write(res.content)

    print(f"downloaded to {args[1]}")


if __name__ == "__main__":
    main()
