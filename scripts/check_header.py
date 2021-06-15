#!/usr/bin/env python

import sys

HEADER_RS = '''//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//
'''

TYPES = {'.rs': HEADER_RS}


def print_indent(text):
    for line in text.splitlines():
        print('    {}'.format(line))


def check_header(name, expected_header):
    with open(name) as src:
        header = ''.join(src.readlines()[:4])
        if header != expected_header:
            print('\nFile {} has invalid header'.format(name))
            print('Found:')
            print_indent(header)
            print('Should be:')
            print_indent(expected_header)
            return False
        else:
            return True


if len(sys.argv) < 2:
    print('Usage:', sys.argv[0], 'git_dir files...')
    sys.exit(1)

gitdir = sys.argv[1]
status = 0
for name in sys.argv[2:]:
    from os import path
    filename = path.normpath(path.join(gitdir, name))
    ext = path.splitext(filename)[1]
    expected_header = TYPES.get(ext)
    if expected_header:
        if not check_header(filename, expected_header):
            status = 100


sys.exit(status)
