#!/usr/bin/python3

# Number of tests for each bench-set
TEST_RETRIES = 5

# File to store the results
FILENAME = "speed.csv"

import argparse
import csv

def parse_args():
    global FILENAME

    parser = argparse.ArgumentParser(description = "SuckIT benchmark")

    parser.add_argument("-f", "--filename", action = "store", type = str, help = f"result file (default_value = '{FILENAME}')")

    args = parser.parse_args()

    if args.filename:
        FILENAME = args.filename

def load_prev_result(filename):
    try:
        with open(filename, "r") as results:
            res = []
            # There has to be at least a line of '1's in the file
            line = results.readlines()[-1]

            reader = csv.reader([line], delimiter = ",")
            for row in reader:
                for cell in row:
                    res.append(int(cell))

    except IOError:
        print(f"Could not read file {filename}")
        exit(1)

    return res

def write_new_result(filename, result):
    try:
        with open(filename, "a") as csv:
            csv.write("{}, {}, {}".format(result[0], result[1], result[2]))
    except IOError:
        print(f"Could not write to file {filename}")
        exit(1)

def compute_new_result():
    return [12, 6, 3]

def main():
    parse_args()

    test_names = ["Single thread", "Two threads", "Four threads"]
    old_result = load_prev_result(FILENAME)

    new_result = compute_new_result()

    for i in range(0, len(test_names)):
        speed_up = new_result[i] * 100 / old_result[i] - 100;

        print(f"{test_names[i]} was {speed_up} quicker")

    write_new_result(FILENAME, new_result)

if __name__ == "__main__":
    main()
