#!/usr/bin/env python3
import os
import sys
from string import Template


def path(day: int) -> (str, str):
    return


def write_day_mod(day: int):
    dir_path = f'lib/solution/src/day{day:02d}'
    file_path =f'lib/solution/src/day{day:02d}/mod.rs'

    if os.path.exists(file_path):
        return

    with open("template/day_mod.rs") as f:
        template = Template(f.read())

    output = template.substitute(day=target_day, padded_day=f'{target_day:02d}')

    print(f"Write {file_path}")

    if not os.path.exists(dir_path):
        os.mkdir(dir_path)

    with open(file_path, 'w', newline='\n', encoding="utf-8") as f:
        f.write(output)


def update_lib(day: int):
    fpath = 'lib/solution/src/lib.rs'
    with open(fpath) as f:
        content = f.read()

    new_content = content.replace(f'//pub mod day{day:02d};', f'pub mod day{day:02d};')

    if content == new_content:
        return

    print(f"Update {fpath}")

    with open(fpath, 'w', newline='\n', encoding="utf-8") as f:
        f.write(new_content)


def update_runner(day: int):
    fpath = 'bin/aoc2022/src/main.rs'
    with open(fpath) as f:
        content = f.read()

    match_line = f'Day::Day{day:02d} => Box::new(solution::day{day:02d}::Solution::new()),'
    marker = f'Day::Day{day:02d} => Box::new(aoc::MissingSolution::new()),'

    if match_line in content:
        return

    print(f'Add Day{day:02d} to aoc2022')

    new_content = content.replace(marker, match_line)

    with open(fpath, 'w', newline='\n', encoding="utf-8") as f:
        f.write(new_content)


if len(sys.argv) < 2:
    print(f"{sys.argv[0]} <day>")
    sys.exit(1)

target_day = int(sys.argv[1])

write_day_mod(target_day)
update_lib(target_day)
update_runner(target_day)
