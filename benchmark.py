"""
benchmark.py - Benchmarking cram (with all algorithms) and tar (with all algorithms) on a directory on a file.

- Runs on 6 benchmarks:
    - 1 KB random file
    - 1 MB random file
    - 10 MB random file
    - github.com/werdl/mintext @ a9dc623b0e71cd021d5536c50fb4cff8bbc9024b - small size directory
    - github.com/spartanproj/os @ 16c6f128e741cd3da7af49b208f147c07948f7c4 - medium size directory
    - github.com/tukaani-project/xz @ 462ca9409940a19f743daee6b3bcc611277d0007 - large size directory
- Runs using the following algorithms:
    - gzip, bzip2, lzma, xz, zstd, brotli
    - on cram, it is included in the binary, but with tar, it is not included in the binary, so it is run using the command line (Unix-like assumed)
"""

import os
import subprocess
import time
import statistics
import json


# First, git clone the repositories, before git checkout the commit

# Create a temporary directory

# Clone the repositories
os.system("git clone https://github.com/werdl/mintext files/mintext")
os.system("git clone https://github.com/spartanproj/os files/os")
os.system("git clone https://github.com/tukaani-project/xz files/xz")

# Checkout the commits
os.system("cd files/mintext && git checkout a9dc623b0e71cd021d5536c50fb4cff8bbc9024b")
os.system("cd files/os && git checkout 16c6f128e741cd3da7af49b208f147c07948f7c4")
os.system("cd files/xz && git checkout 462ca9409940a19f743daee6b3bcc611277d0007")

# The random files are already generated - to ensure consistency

# Benchmarking
def benchmark(command: str) -> tuple[float, int]: # (time, size)
    # average of 5 runs
    times = []
    for i in range(5):
        start = time.time()
        os.system(command)
        end = time.time()
        times.append(end - start)

    return statistics.fmean(times)

def tar(algorithm: str, file: str) -> tuple[float, int]:
    EXECUTABLES = {
        "gzip": "gzip",
        "bzip2": "bzip2",
        "lzma": "xz",
        "zstd": "zstd",
        "brotli": "brotli"
    }

    # first, tar the file
    command_tar = f"tar -cf files/{file}.tar files/{file}"

    # then, compress the tar file
    command_comp = f"{EXECUTABLES[algorithm]} files/{file}.tar"


    return benchmark(f"{command_tar} && {command_comp} && rm files/{file}.tar*")

def cram(algorithm: str, file: str) -> tuple[float, int]:
    command_cram = f"target/debug/cram pack -c {algorithm} files/{file}"
    return benchmark(command_cram)

ALGORITHMS = ["gzip", "bzip2", "lzma", "zstd"]
FILES = ["1KB_RANDOM", "1MB_RANDOM", "10MB_RANDOM", "mintext", "os", "xz"]

tar_res = {}
cram_res = {}

for file in FILES:
    tar_res[file] = {}
    cram_res[file] = {}

    for algorithm in ALGORITHMS:
        print("Benchmarking", file, algorithm)
        tar_res[file][algorithm] = tar(algorithm, f"{file}")
        cram_res[file][algorithm] = cram(algorithm, f"{file}")

with open("benchmark.json", "w") as f:
    json.dump({
        "tar": tar_res,
        "cram": cram_res
    }, f)
end = {
        "tar": tar_res,
        "cram": cram_res
    }

