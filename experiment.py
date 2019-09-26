#!/usr/bin/env python3

import os
from os import path
import subprocess as sub
import statistics

DATA_DIR = 'pp-data'
SEED_LEN = 128
MAX_LEN = 2048
N = 1000000
B_VALUES = list(range(SEED_LEN, MAX_LEN + 1, SEED_LEN))
GEN_PGM = './gen/target/release/gen'
GCD_PGM = './fastgcd'
OUT = 'out.txt'

rngs = [
    ('plus-prime', ['plus-prime', '-s', f'{SEED_LEN}']),
    ('uniform', ['uniform']),
    ('inc', ['permutation', '-p', 'increment', '-s', f'{SEED_LEN}']),
]

assert path.exists(GEN_PGM)
assert path.exists(GCD_PGM)

def analyze(directory):
    with open(f'{directory}/input.txt', 'r') as f:
        n = len(f.readlines())
    with open(f'{directory}/gcds', 'r') as f:
        gcds = [int(l, 16) for l in f.readlines()]
    for _ in range(0, n - len(gcds)):
        gcds.append(1)
    bitlengths = [len(f'{g:b}') for g in gcds]
    return ( statistics.mean(bitlengths), statistics.stdev(bitlengths), max(bitlengths) )

with open(OUT, 'w') as of:
    of.write('name\tb\ts\tmean\tstdev\tmax\n')
    for b in B_VALUES:
        for (name, args) in rngs:
            d = f'{DATA_DIR}/{name}/{b}'
            infile = f'{d}/input.txt'
            os.makedirs(d, exist_ok = True)
            with open(infile, 'w') as f:
                sub.run([GEN_PGM] + ['-n', f'{N}', '-b', f'{b}'] + args, stdout=f)
            sub.run([f'../../../{GCD_PGM}', 'input.txt'], cwd=d)
            mean, stdev, max_ = analyze(d)
            of.write(f'{name}\t{b}\t{SEED_LEN}\t{mean}\t{stdev}\t{max_}\n')
            of.flush()

