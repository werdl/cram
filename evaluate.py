# now evaulate the data for the user, and plot it on a matplotlib graph (ensure to save the graph as a file)
# the user should be able to see the data in a table format
# the user should be able to see the data in a graph format


ALGORITHMS = ["gzip", "bzip2", "lzma", "zstd"]
FILES = ["1KB_RANDOM", "1MB_RANDOM", "10MB_RANDOM", "mintext", "os", "xz"]

import json
import os

with open("benchmark.json", "r") as f:
    data = json.load(f)

tar_res = data["tar"]
cram_res = data["cram"]

tar_wins = 0
cram_wins = 0

for file in FILES:
    for algorithm in ALGORITHMS:
        print("File:", file, "Algorithm:", algorithm)
       
        winner = "tar" if tar_res[file][algorithm] < cram_res[file][algorithm] else "cram"

        percent_diff = abs(tar_res[file][algorithm] - cram_res[file][algorithm]) / tar_res[file][algorithm] * 100

        if winner == "tar":
            tar_wins += 1
        else:
            cram_wins += 1

        print(f"Winner: {winner} ({percent_diff:.2f}% difference)")

print("Tar wins:", tar_wins)
print("Cram wins:", cram_wins)

def get_size(start_path = '.'):
    total_size = 0
    for dirpath, dirnames, filenames in os.walk(start_path):
        for f in filenames:
            fp = os.path.join(dirpath, f)
            # skip if it is symbolic link
            if not os.path.islink(fp):
                total_size += os.path.getsize(fp)

    return total_size

# now plot the data
import matplotlib.pyplot as plt
import numpy as np

for file in FILES:
    fig, ax = plt.subplots()

    file_size = os.path.getsize(f"files/{file}") / 1024 / 1024

    if file_size == 0 or os.path.getsize(f"files/{file}") == 4096:
        # probably a directory
        file_size = get_size(f"files/{file}") / 1024 / 1024

    if file_size < 1:
        file_size *= 1024
        ax.set_title(
            file + f" ({file_size:.2f}KB)"
        )
    else:

        ax.set_title(
            file + f" ({file_size:.2f}MB)"
        )
    ax.set_ylabel("Time (s)")
    ax.set_xlabel("Algorithm")

    x = np.arange(len(ALGORITHMS))
    width = 0.35

    tar_times = [tar_res[file][algorithm] for algorithm in ALGORITHMS]
    cram_times = [cram_res[file][algorithm] for algorithm in ALGORITHMS]

    ax.bar(x - width/2, tar_times, width, label="tar")
    ax.bar(x + width/2, cram_times, width, label="cram")

    ax.set_xticks(x)
    ax.set_xticklabels(ALGORITHMS)

    ax.legend()

    plt.savefig(f"benchmark/{file}.png")