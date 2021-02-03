#!/usr/bin/env python3

import csv
import argparse

opcodes = csv.reader(open("opcodes.tsv"), delimiter="\t")
addressing = csv.reader(open("addressing.tsv"), delimiter="\t")
bytelen = csv.reader(open("bytelength.tsv"), delimiter="\t")
cyclelen = csv.reader(open("cyclelength.tsv"), delimiter="\t")

table = [[{} for _ in range(16) ] for _ in range(16)]

for i, row in enumerate(opcodes):
    for j, cell in enumerate(row):
        table[j][i]['opcode'] = cell

for i, row in enumerate(addressing):
    for j, cell in enumerate(row):
        table[j][i]['addressing'] = cell

for i, row in enumerate(bytelen):
    for j, cell in enumerate(row):
        table[j][i]['bytelen'] = cell

for i, row in enumerate(cyclelen):
    for j, cell in enumerate(row):
        table[j][i]['cyclelen'] = cell

def check(table):
    for j in range(len(table)):
        for i in range(len(table[0])):
            print('===============================================')
            op = table[i][j]
            if op:
                print(op['opcode'])
                print(op['addressing'])
                print(op['bytelen'], '  ', op['cyclelen'])
            else:
                print('\n\n')

            input()

def create_enum(table):

    adrs = {
        'accum': 'acc',
        'imm': 'imm',
        'abs': 'abs',
        'zp': 'zp',
        'zp, x': 'zp_x',
        'zp, y': 'zp_y',
        'abs, x': 'abs_x',
        'abs, y': 'abs_y',
        'implied': 'imp',
        'relative': 'rel',
        '(ind, x)': 'ind_x',
        '(ind), y': 'ind_y',
        'indirect': 'ind',
    }

    print('#[repr(u8)]')
    print('pub enum Opcodes {')
    for j in range(len(table)):
        for i in range(len(table[0])):
            op = table[i][j]
            if op and op['opcode']:
                name = op['opcode'].upper() + '_' + adrs[op['addressing']]
                byte = hex(j) + hex(i)[2]
                print(f'    {name} = {byte},')
    print('}')

p=argparse.ArgumentParser()
p.add_argument('mode', choices=['check','enum'])
args = p.parse_args()

if args.mode == 'check':
    check(table)
elif args.mode == 'enum':
    create_enum(table)