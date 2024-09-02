#!/usr/bin/env python3


import os.path, time
import sys
if sys.version_info[0] == 2:
    MODE_RU = 'rU'
else:
    MODE_RU = 'r'

import Krakatau
from Krakatau.assembler import parse
from Krakatau import script_util

def assembleSource(source, basename, fatal=False):
    source = source.replace('\t', '  ') + '\n'
    return list(parse.assemble(source, basename, fatal=fatal))

def assembleClass(filename):
    basename = os.path.basename(filename)
    try:
        with open(filename, MODE_RU) as f:
            source = f.read()
    except Exception:
        with open(filename, MODE_RU, encoding='utf8') as f:
            source = f.read()
    return assembleSource(source, basename)

if __name__== "__main__":
    import argparse
    parser = argparse.ArgumentParser(description='Krakatau bytecode assembler')
    parser.add_argument('-out', help='Path to generate files in')
    parser.add_argument('-r', action='store_true', help="Process all files in the directory target and subdirectories")
    parser.add_argument('-q', action='store_true', help="Only display warnings and errors")
    parser.add_argument('target', help='Name of file to assemble')
    args = parser.parse_args()

    log = script_util.Logger('warning' if args.q else 'info')
    log.info(script_util.copyright)

    out = script_util.makeWriter(args.out, '.class')
    targets = script_util.findFiles(args.target, args.r, '.j')

    start_time = time.time()
    with out:
        for i, target in enumerate(targets):
            log.info('Processing file {}, {}/{} remaining'.format(target, len(targets)-i, len(targets)))

            pairs = assembleClass(target)
            for name, data in pairs:
                filename = out.write(name, data)
                log.info('Class written to', filename)
    print('Total time', time.time() - start_time)
