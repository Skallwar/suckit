#!/usr/bin/python3

from termcolor import colored
from tabulate import tabulate
from threading import Thread

import time
import os

# Set the running time to 2 minutes for both benchmarks
RUN_TIME = 120

def print_info():
    info = """
    This benchmark takes approximately 4 minutes to run.
    It aims to bench suckit against other, popular website
    downloaders such as httrack
    """

    time_str = """
    Each program will run for {} seconds
    """.format(RUN_TIME)

    print(f"{colored(info, 'blue')}")
    print(time_str)

def bench_worker(cmd):
    os.system("mkdir " + cmd)
    os.system("cd  " + cmd)
    os.system(cmd + " https://wikipedia.org/wiki/Mushroom")

def bench(cmd):
    thread = Thread(target = bench_worker, args = (cmd, ))
    thread.start()

    # Let the benched program run for a certain amount of time
    time.sleep(RUN_TIME)

    thread.join()

    # Count the number of files it downloaded
    count = sum([len(files) for r, d, files in os.walk(".")])

    # Go back to /tmp
    os.system("cd ..")

    return count

def flush_output(res):
    print(tabulate(res, headers = ["name", "pages downloaded"]))

def main():
    print_info()

    os.system("cd /tmp/")

    results = []

    results.append(["suckit", bench("suckit")])
    results.append(["httrack", bench("httrack")])

    flush_output(results)

if __name__ == "__main__":
    main()
