#!/usr/bin/python3

# Number of tests for each bench-set
TEST_RETRIES = 5

# File to store the results
FILENAME = "speed.csv"

# Path to the suckit binary
SUCKIT = "suckit"

# URL to download
URL = "https://books.toscrape.com"

# Path to store the downloaded data
PATH = "/tmp/suckit_speed"

import argparse
import csv
import os
import shutil
import subprocess
import time

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

def load_prev_result(filename):
    try:
        with open(filename, "r") as results:
            res = []
            # There has to be at least a line of '1's in the file
            line = results.readlines()[-1]

            reader = csv.reader([line], delimiter = ",")
            for row in reader:
                for cell in row:
                    res.append(float(cell))

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
        start_time = time.time()

        print(f"-j {count}")
        print(f"-o {PATH}")
        subprocess.Popen([SUCKIT, "-j {}".format(count), "-o {}".format(PATH), URL],
                stdout = open("/dev/null", "w"), shell = False).pid

        end_time = time.time()

        res.append(end_time - start_time)

    return res

def main():
    parse_args()

    # If the directory already exists, just ignore it
    try:
        os.mkdir(PATH)
    except OSError:
        pass

    test_names = ["Single thread", "Two threads", "Four threads"]
    old_result = load_prev_result(FILENAME)

    new_result = compute_new_result()

    for i in range(0, len(test_names)):
        speed_up = new_result[i] * 100 / old_result[i] - 100;

        print(f"{test_names[i]} was {speed_up} quicker")

    write_new_result(FILENAME, new_result)

    shutil.rmtree(PATH)

if __name__ == "__main__":
    main()
