#!/usr/bin/python3

# Number of tests for each bench-set
TEST_RETRIES = 20

# File to store the results
FILENAME = "speed.csv"

# Path to the suckit binary
SUCKIT = "suckit"

# URL to download: localhost
URL = "http://0.0.0.0"

# Path to store the downloaded data
PATH = "/tmp/suckit_speed"

import argparse
import csv
import os
import shutil
import subprocess
import time
from termcolor import colored

def parse_args():
    global FILENAME
    global SUCKIT

    parser = argparse.ArgumentParser(description = "SuckIT benchmark")

    parser.add_argument("-f", "--filename", action = "store", type = str, help = f"result file (default_value = '{FILENAME}')")
    parser.add_argument("-s", "--suckit", action = "store", type = str, help = f"path to the suckit binary (default_value = '{SUCKIT}')")

    args = parser.parse_args()

    if args.filename:
        FILENAME = args.filename

    if args.suckit:
        SUCKIT = args.suckit

def load_best_result(filename):
    try:
        with open(filename, "r") as results:
            res = [float('inf'), float('inf'), float('inf')]
            # There has to be at least a line of '1's in the file
            line = results.readlines()[-1]

            reader = csv.reader([line], delimiter = ",")
            for row in reader:
                for i in range(len(row)):
                    candidate = float(row[i])
                    if candidate < res[i]:
                        res[i] = candidate

    except IOError:
        print(f"Could not read file {filename}")
        exit(1)

    return res

def write_new_result(filename, result):
    try:
        with open(filename, "a") as csv:
            csv.write("{}, {}, {}\n".format(result[0], result[1], result[2]))
    except IOError:
        print(f"Could not write to file {filename}")
        exit(1)

def compute_new_result():
    thread_counts = ["1", "2", "4"]
    res = []

    for count in thread_counts:
        time_total = 0
        for i in range(TEST_RETRIES):
            start_time = time.time()

            suckit_pid = subprocess.Popen([SUCKIT, "-j", count, "-o", PATH, URL],
                    stdout = open("/dev/null", "w"), shell = False)
            suckit_pid.wait()

            end_time = time.time()

            time_total += end_time - start_time
            print(f"Completed {i + 1} iteration for job with {count} thread(s)", end = "\r")

        res.append(time_total / TEST_RETRIES)
        print("")

    return res

def main():
    parse_args()

    # If the directory already exists, just ignore it
    try:
        os.mkdir(PATH)
    except OSError:
        pass

    test_names = ["Single thread", "Two threads", "Four threads"]
    old_result = load_best_result(FILENAME)

    new_result = compute_new_result()

    for i in range(0, len(test_names)):
        speed_up = new_result[i] * 100 / old_result[i] - 100;

        str_speed_up = f"{colored(speed_up, 'green')}"

        if speed_up > 0:
            str_speed_up = f"{colored(speed_up, 'red')}"

        print(f"{test_names[i]} was {str_speed_up} slower")

    write_new_result(FILENAME, new_result)

    shutil.rmtree(PATH)

if __name__ == "__main__":
    main()
