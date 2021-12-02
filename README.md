# adventofcode

A runner for AoC and solutions for every year  (hopefully).

Requires session cookie in ```.env``` file in the form of ```SESSION=X```. 

## Usage 

Can run a specific day or a whole year. Without any input, it defaults to last available aoc year and day.

```
aoc 0.1.0
AoC runner.

USAGE:
    adventofcode [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        Run all solutions for a given year
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <day>      Which day to run (default: last available aoc day)
    -y, --year <year>    Which year to run (default: last available aoc year)
```