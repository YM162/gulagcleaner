
# Gulag Cleaner Python distribution

![PyPI - Downloads](https://img.shields.io/pypi/dm/gulagcleaner)

# Installation

To install Gulag Cleaner, please [download](https://www.python.org/downloads/) and [install](https://wiki.python.org/moin/BeginnersGuide/Download) Python and then run the following command in your terminal:
```
pip install gulagcleaner
```

# Usage

Gulag Cleaner can be used through both a Command Line Interface (CLI) and in your code.

## Command Line Interface

To use Gulag Cleaner through the CLI, simply run the following command, replacing `<filename>` with the name of one or more PDF files or folders containing PDF:

```
gulagcleaner [-r] [-s] [-n] [-h] [-v] <filename>...
```

## Options

Gulag Cleaner provides several options for its usage:

> * '-r': Replace the original file with the cleaned version.
> * '-s': Do not show metadata about cleaned files.
> * '-n': Force the naive cleaning method.
> * '-h': Display the help message, providing information on how to use Gulag Cleaner.
> * '-v': Display the current version of Gulag Cleaner.

## Code

To use Gulag Cleaner in your code, you can use the following code snippet:

```python
from gulagcleaner.extract import clean_pdf

return_msg = clean_pdf_path("input.pdf","output.pdf")
```