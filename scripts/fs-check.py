#!/usr/bin/env python3

from pathlib import Path
import os
import sys

print(" - We happy?")

root = os.path.join('lib', 'solution', 'src')

for solution in Path(root).iterdir():
    if not solution.is_dir():
        continue


    if not os.path.exists(os.path.join(solution, "readme.md")):
        day=solution.name.replace('day', '')
        print(f"{solution} has not generated readme; use 'make readme_day{day}'")
        sys.exit(1)

print(" - Yeah, we happy.")

sys.exit(0)
