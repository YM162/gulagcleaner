from gulagcleaner.clean import clean_pdf_path
from gulagcleaner.decrypt import decrypt_pdf
from gulagcleaner.metadata import extract_metadata
from os.path import exists, isdir, join
from os import listdir, remove

def parseArgs():
    '''
    Function to parse arguments.

    Checks for any optional arguments passed to the program and activates
    the corresponding flags.
    '''
    from sys import argv
    targeted = ['-h', '-r', '-s', '-v','-n', argv[0]]

    return {
        'help': '-h' in argv,
        'replace': '-r' in argv,
        'short': '-s' in argv,
        'version': '-v' in argv,
        'force_naive': '-n' in argv,
        'files': [arg for arg in argv if arg not in targeted]
    }

def main():
    '''
    Main function for the "gulagcleaner" CLI command.

    The "gulagcleaner" command takes arguments for the path of one or more 
    files which can be PDF files or folders containing PDFs, and tries to 
    remove the ads inside of them. The new PDFs are saved in their original 
    location.
    
    Available CLI arguments:
    -r : Replace original files with their cleaned version.
    -s : Do not show metadata about cleaned files.
    -n : Force the naive cleaning method.
    -h : Display help information.
    -v : Display the version of the program.

    '''
    arguments = parseArgs()

    # Check for the -h argument
    if arguments["help"]:
        print("Usage: gulagcleaner [-r] [-s] [-n] [-h] [-v] <pdf_path>")
        print("")
        print("Removes ads from PDF files.")
        print("")
        print("Positional arguments:")
        print("  pdf_path      PDF file to clean.")
        print("")
        print("Optional arguments:")
        print("  -r            Replace original files with their cleaned version.")
        print("  -s            Do not show metadata about cleaned files.")
        print("  -n            Force the naive cleaning method.")
        print("  -h            Show this help message.")
        print("  -v            Show the version of the program.")
        return

    # Check for the -v argument
    if arguments["version"]:
        print("Current version: 0.14.2")
        return

    # Get the pdf_path argument
    if len(arguments["files"]) == 0:
        print('Usage: gulagcleaner [-r] [-s] [-n] [-h] [-v] <pdf_path>...')
        return
    
    replace = arguments["replace"]
    short = arguments["short"]
    force_naive = arguments["force_naive"]
    for element in arguments["files"]:
        # Check if the file exists
        if not exists(element):
            print(element + " not found.")
            continue
        
        # Check if file is a directory
        if isdir(element):
            # Add PDF files of directory to list of files to clean
            arguments["files"] += [join(element, file) for file in listdir(element) 
                                   if file.endswith('.pdf') or isdir(join(element, file))]
            continue

        pdf_path = element
        
        if replace:
            output_path = pdf_path
        else:
            output_path = pdf_path[:-4] + "_clean.pdf"
        
        # We decrypt the PDF file
        pdf_path = decrypt_pdf(pdf_path)

        # If short mode is not active, extract metadata
        if not short:
            try:
                metadict = extract_metadata(pdf_path)
                print("Metadata:")
                print("Archivo: " + metadict["Archivo"])
                print("Autor: " + metadict["Autor"])
                print("Asignatura: " + metadict["Asignatura"])
                print("Curso y Grado: " + metadict["Curso y Grado"])
                print("Facultad: " + metadict["Facultad"])
                print("Universidad: " + metadict["Universidad"])
            except Exception as e:
                print("Failed to extract metadata:", e)

        # Call the cleaning function
        return_msg = clean_pdf_path(pdf_path, output_path, force_naive)
        remove(pdf_path)
        if return_msg["success"]:
            print("Cleaning successful. File saved in " + 
                  return_msg["return_path"])
        else:
            print("Error cleaning " + pdf_path + ": " + return_msg["error"])

if __name__ == "__main__":
    print('Call from the "gulagcleaner" command.')
