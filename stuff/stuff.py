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

def addrname(addr):
    return adrs[addr]

def fulladdrname(addr):
    adrs = {
        'accum': 'Accum',
        'imm': 'IMM',
        'abs': 'Absolute',
        'zp': 'ZP',
        'zp, x': 'ZP, X',
        'zp, y': 'ZP, Y',
        'abs, x': 'ABS, X',
        'abs, y': 'ABS, Y',
        'implied': 'Implied',
        'relative': 'Relative',
        '(ind, x)': '(IND, X)',
        '(ind), y': '(IND), Y',
        'indirect': 'Indirect',
    }
    return adrs[addr]

def operand(addrname):
    if addrname in ['imp', 'acc']:
        return 'Implied'
    elif addrname == 'imm':
        return 'Immediate'
    else:
        return 'Address'

def funname(op):
    return op['opcode'].lower() + '_' + operand(addrname(op['addressing']))

def opname(op):
    return op['opcode'].upper() + '_' + addrname(op['addressing'])

def create_enum(table):
    print('#[repr(u8)]')
    print('pub enum Opcodes {')
    for j in range(len(table)):
        for i in range(len(table[0])):
            op = table[i][j]
            if op and op['opcode']:
                name = opname(op)
                byte = hex(j) + hex(i)[2]
                print(f'    {name} = {byte},')
    print('}')

def create_opmatch(table):
    print('match self {')
    
    for j in range(len(table)):
        for i in range(len(table[0])):
            op = table[i][j]
            if op and op['opcode']:
                arm = (
                    f'    \n    Opcode::{opname(op)} => {{\n'
                    f'        &Instruction{{\n'
                    f'            cycles: {op["cyclelen"].strip("*")},\n'
                    f'            bytes: {op["bytelen"]},\n'
                    f'            addr_delegate: addressing::{addrname(op["addressing"])},\n'
                    f'            op_delegate: OpDelegate::{operand(addrname(op["addressing"]))}(ops::{funname(op).lower()}),\n'
                    f'            mnemonic: "{op["opcode"].upper()}",\n'
                    f'            addressing: "{fulladdrname(op["addressing"])}",\n'
                    f'        }}\n'
                    f'    }}'
                )

                print(arm)

    print('}')

def create_funstumps(table):
    sigs = {
        'Implied': 'pub fn {funname}(regs: &mut CPURegisters, cycle: usize) -> BusMessage {{\n',
        'Immediate': 'pub fn {funname}(regs: &mut CPURegisters, immediate: u8, _cycle: usize) -> BusMessage {{\n',
        'Address': 'pub fn {funname}(regs: &mut CPURegisters, address: u16, cycle: usize) -> BusMessage {{\n',
    }

    funs = set()

    for j in range(len(table)):
        for i in range(len(table[0])):
            op = table[i][j]
            if op and op['opcode']:
                fun = (
                    '\n'
                    + sigs[operand(addrname(op['addressing']))] +
                    '    todo!("functionality for {funname}()");\n'
                    '}}'
                ).format(funname=funname(op).lower())

                funs.add(fun)

    for fun in funs:
        print(fun)

def create_addrfunstumps():
    for key, value in adrs.items():
        fun = (
            'pub fn {funname}(regs: &mut CPURegisters, cycle: usize) -> AddrDelegateReturn {{\n'
            '    todo!("functionality for {funname}() addressing");\n'
            '}}\n'
        ).format(funname=value)
        
        print(fun)


p=argparse.ArgumentParser()
p.add_argument('mode', choices=['check','enum', 'opmatch', 'funstumps', 'addrfunstumps'])
args = p.parse_args()

if args.mode == 'check':
    check(table)
elif args.mode == 'enum':
    create_enum(table)
elif args.mode == 'opmatch':
    create_opmatch(table)
elif args.mode == 'funstumps':
    create_funstumps(table)
elif args.mode == 'addrfunstumps':
    create_addrfunstumps()