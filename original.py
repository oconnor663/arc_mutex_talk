#! /usr/bin/env python3

import threading
import time


def main():
    number = [0]
    mutex = threading.Lock()
    threading.Thread(
        target=add_loop,
        args=(number, mutex),
        daemon=True,
    ).start()
    while True:
        with mutex:
            print(number[0])
        time.sleep(1)


def add_loop(number, mutex):
    while True:
        with mutex:
            number[0] += 1


if __name__ == "__main__":
    main()
