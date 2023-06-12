from gulagcleaner.extract import deembed
from gulagcleaner.decrypt import decrypt_pdf
from gulagcleaner.metadata import extract_metadata
from os.path import exists

def main():
    '''
    Main function for the "gulagcleaner" CLI command.

    The "gulagcleaner" command takes an argument for the path of a PDF file and tries to deembed the pages inside it. The pages are saved in a new PDF in the same folder.
    
    Available CLI arguments:
    -h : Display help information
    -r : Replace the original file with the deembedded file
    -o : Use the old deembeding method (for files older than 18/05/2023)
    -v : Display the version of the program

    '''
    import sys
    import os.path

    # Check for the -h argument
    if '-h' in sys.argv:
        print("Usage: gulagcleaner [-h] [-r] [-o] [-v] <filename>")
        print("")
        print("Deembeds pages from a PDF file.")
        print("")
        print("Positional arguments:")
        print("  filename      The PDF file to deembed pages from.")
        print("")
        print("Optional arguments:")
        print("  -h            Show this help message.")
        print("  -r            Replace the original file with the deembedded file.")
        print("  -o            Use the old deembeding method (for files older than 18/05/2023).")
        print("  -v            Show the version of the program.")
        return

    # Check for the -v argument
    if '-v' in sys.argv:
        print("Actual version: 0.5.2")
        return

    # Get the filename argument
    if len(sys.argv) < 2:
        print('Usage: gulagcleaner [-h] [-r] [-o] [-v] <filename>')
        return
    filename = sys.argv[-1]

    # Check if the file exists
    if not os.path.exists(filename):
        print("File not found.")
        return

    # Check if the -r argument is present
    replace = '-r' in sys.argv

     # Check if the -o argument is present
    if '-o' in sys.argv:
        method = "old"
        filename = decrypt_pdf(filename)
        intermediate = True
    else:
        method = "new"
    
    # Call the deembed function
    return_msg = deembed(filename, replace, method)
    if intermediate:
        os.remove(filename)

    if return_msg["Success"]:
        print("Deembedding successful. File saved in", return_msg["return_path"])
        try:
            metadict = extract_metadata(filename)
            print("Metadata:")
            print("Archivo: " + metadict["Archivo"])
            print("Autor: " + metadict["Autor"])
            print("Asignatura: " + metadict["Asignatura"])
            print("Curso y Grado: " + metadict["Curso y Grado"])
            print("Facultad: " + metadict["Facultad"])
            print("Universidad: " + metadict["Universidad"])
        except Exception as e:
            print("Failed to extract metadata:", e)            
    else:
        print("Error:", return_msg["Error"])

if __name__ == "__main__":
    print('Call from the "gulagcleaner" command.')
