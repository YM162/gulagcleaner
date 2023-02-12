# Gulag Cleaner

Gulag Cleaner is a tool designed to remove advertisements from PDFs, making it easier to read and navigate documents without being disrupted by unwanted ads. It operates as a functional inverse of functions that insert PDF pages into other documents as Form XObjects, and is particularly useful for reversing the effects of the embedPages() function from the pdf-lib.js library.

In addition to removing advertisements, Gulag Cleaner is also capable of extracting metadata, such as the author, subject, university, and more, from the file.

## Installation

To install Gulag Cleaner, simply run the following command in your terminal:
```
pip install gulagcleaner
```

## Usage

Gulag Cleaner can be used through both a Command Line Interface (CLI) and in your code.

### Command Line Interface

To use Gulag Cleaner through the CLI, simply run the following command, replacing `<filename>` with the name of your PDF file:

```
gulagcleaner <filename> [-r] [-h] [-v]
```

### Code

To use Gulag Cleaner in your code, you can use the following code snippet:

```python
from gulagcleaner.gulagcleaner_extract import deembed

return_msg = deembed("file.pdf")
```

## Options

Gulag Cleaner provides several options for it's usage:

> * '-r': Replaces the original file with the cleaned version.
> * '-h': Displays the help message, providing information on how to use Gulag Cleaner.
> * '-v': Displays the current version of Gulag Cleaner.

## License
Gulag Cleaner is distributed under the GPL-3 license, which means it's open-source and free to use.

## Contributing
We're always looking for ways to improve Gulag Cleaner, and we welcome contributions from the community. If you have ideas for improvements or bug fixes, please feel free to submit a pull request.