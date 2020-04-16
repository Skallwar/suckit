#!/usr/bin/python3

# Number of tests for each bench-set
TEST_RETRIES = 5

# File to store the results
FILENAME = "speed.csv"

def parse_args():
    global FILENAME

    parser = argparse.ArgumentParser(description = "SuckIT benchmark")

def main():
    test_names = ["Single thread", "Two threads", "Four threads"]
    old_result = load_prev_result(filename)

    new_result = compute_new_result()

    for i in range(test_names):
        speed_up = new_result[i] * 100 / old_result[i]

        print("{} was {} quicker", test_names[i], speed_up)

if __name__ == "__main_":
    main()
