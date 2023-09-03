## How it works

This utility returns the result of concatenating two csv files by the specified column as a csv with a set of selected columns from each csv

## How to use

`csvmerge -s 0 -d 1 -r "0,1,2 0,2,3" /path/to/file1.csv /path/to/file2.csv`

## Options

- `-s`, `--source` - column number to search in the first file
- `-d`, `--destination` - column number to search in the second file
- `-r`, `--result` - list of columns to be taken from the first and second files, respectively. Separated by a comma between themselves and a space between files
- `-o`, `--output` - path to result file
