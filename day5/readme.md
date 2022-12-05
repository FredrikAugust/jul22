# unnskyld

beklager for kodekvaliteten her

## det jeg skulle gjort

- split on `\n\n`

### for stack setup (first block)

- split on `\n`
- transpose
- only keep lines `4n+1`
- iterate over new lines
  - first char is stack number, subsequent chars are items

### instructions

#### part1

for each count just pop and push, works fine. performance isn't an issue here

#### part2

copy out the last count items and remove them from `from`-stack and push in same order to `to`
