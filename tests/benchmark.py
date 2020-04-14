#!/usr/bin/python3

from termcolor import colored
from tabulate import tabulate
from threading import Thread

import time
import argparse
import sys
import errno
import os
import shutil
import subprocess
import signal

# Output directory for the downloaded files. Cleaned afterwards
# Be extra careful when changing that value !
OUTPUT_DIR = "/tmp/suckit_bench"

# Set the running time for both benchmarks
RUN_TIME = 120

# Keep track of the current PID to SIGINT it
CUR_PID = 0

# Path to the suckit binary
SUCKIT_CMD = "suckit"

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

def bench_worker(dir_name, cmd):
    global CUR_PID

    # Handle the case where the directory exists already
    try:
        os.mkdir(dir_name)
    except OSError as exc:
        if exc.errno != errno.EEXIST:
            raise
        pass

    os.chdir(dir_name)

    CUR_PID = subprocess.Popen([cmd, "https://forum.httrack.com"],
            stdout = open("/dev/null", "w"), shell = False).pid

def bench(dir_name, cmd):
    thread = Thread(target = bench_worker, args = (dir_name, cmd, ))
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
    parser = argparse.ArgumentParser(description = "SuckIT benchmark")
    parser.add_argument("-o", "--output", action = "store", type = str, help = "benchmark output directory (default_value = '/tmp/suckit_bench/')")
    parser.add_argument("-t", "--time", action = "store", type = int, help = "time given to each binary in seconds (default_value = 120)")
    parser.add_argument("-s", "--suckit", action = "store", type = str, help = "path to the suckit binary (default_value = 'suckit')")

    args = parser.parse_args()

    global OUTPUT_DIR
    global RUN_TIME
    global SUCKIT_CMD

    if args.output:
        OUTPUT_DIR = args.output

    if args.time:
        RUN_TIME = args.time

    if args.suckit:
        SUCKIT_CMD = os.path.abspath(args.suckit)

    print_info()

    # Handle the case where the directory exists already
    try:
        os.mkdir(OUTPUT_DIR)
    except OSError as exc:
        err ="""
            You're trying to use an already existing directory as your
            output directory. Since the directory will be counted and
            removed after the benchmark, I can't let you do that !
            """
        print(f"{colored(err, 'red')}")
        raise

    os.chdir(OUTPUT_DIR)

    results = []

    results.append(["suckit", bench("suckit", SUCKIT_CMD)])
    results.append(["httrack", bench("httrack", "httrack")])

    flush_output(results)

    # Clean benchmark output
    shutil.rmtree(OUTPUT_DIR)

if __name__ == "__main__":
    main()
