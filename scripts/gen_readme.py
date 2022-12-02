#!/usr/bin/env python3

import subprocess
import sys
from string import Template


def gen(day: int, part: int) -> str:
    cmd = f"cargo run --quiet -- --day={day} --part={part} --time-it"
    result = subprocess.run(cmd.split(" "), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    return clear(result.stdout.decode('utf-8'))


def clear(content: str) -> str:
    lines = content.splitlines()
    while lines and not lines[-1].strip():
        lines.pop()
    return '\n'.join(lines)


def path(day: int) -> str:
    return f'lib/solution/src/day{day:02d}/readme.md'


if len(sys.argv) < 2:
    print(f"{sys.argv[0]} <day>")
    sys.exit(1)

target_day = int(sys.argv[1])
part1 = gen(target_day, 1)
part2 = gen(target_day, 2)

with open("template/day_readme.md") as f:
    template = Template(f.read())
    output = template.substitute(day=target_day, part1=part1, part2=part2)

with open(path(target_day), 'w', newline='\n', encoding="utf-8") as f:
    f.write(output)
