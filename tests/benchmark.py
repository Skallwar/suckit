#!/usr/bin/python3

from termcolor import colored
from tabulate import tabulate
from threading import Thread

import time
import sys
import errno
import os
import shutil
import subprocess
import signal

# Output directory for the downloaded files. Cleaned afterwards
# Be extra careful when changing that value !
OUTPUT_DIR = "/tmp/suckit_bench/"

# Set the running time for both benchmarks
RUN_TIME = int(sys.argv[1])

# Keep track of the current PID to SIGINT it
CUR_PID = 0

def print_info():
    info = """
    This benchmark aims to bench suckit against other, popular website
    downloaders such as httrack
    """

    time_str = """
    Each program will run for {} seconds
    """.format(RUN_TIME)

    print(f"{colored(info, 'blue')}")
    print(time_str)

def bench_worker(cmd):
    global CUR_PID

    # Handle the case where the directory exists already
    try:
        os.mkdir(cmd)
    except OSError as exc:
        if exc.errno != errno.EEXIST:
            raise
        pass

    os.chdir(cmd)

    CUR_PID = subprocess.Popen([cmd, "https://forum.httrack.com"],
            stdout = open("/dev/null", "w"), shell = False).pid

def bench(cmd):
    thread = Thread(target = bench_worker, args = (cmd, ))
    thread.start()

    # Let the benched program run for a certain amount of time
    time.sleep(RUN_TIME)

    thread.join()
    os.kill(CUR_PID, signal.SIGINT)

    # Count the number of files it downloaded
    count = sum([len(files) for r, d, files in os.walk(".")])

    # Go back to /tmp
    os.chdir(OUTPUT_DIR)

    return count

def flush_output(res):
    print(tabulate(res, headers = ["name", "pages downloaded"]))

def main():
    print_info()

    # Handle the case where the directory exists already
    try:
        os.mkdir(OUTPUT_DIR)
    except OSError as exc:
        if exc.errno != errno.EEXIST:
            raise
        pass
    os.chdir(OUTPUT_DIR)

    results = []

    results.append(["suckit", bench("suckit")])
    results.append(["httrack", bench("httrack")])

    flush_output(results)

    # Clean benchmark output
    shutil.rmtree(OUTPUT_DIR)

if __name__ == "__main__":
    main()
