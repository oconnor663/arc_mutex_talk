#! /usr/bin/env python3

import threading
import time


def main():
    output = []
    mutex = threading.Lock()
    threading.Thread(
        target=fill_vector,
        args=(output, mutex),
        daemon=True,
    ).start()
    while True:
        with mutex:
            print(output)
        time.sleep(0.1)


def fill_vector(output, mutex):
    for i in range(2**64):
        with mutex:
            output.append(i)
        time.sleep(1)


if __name__ == "__main__":
    main()
