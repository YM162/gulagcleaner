from gulagcleaner import gulagcleaner_extract
from os.path import exists

def main():
    '''
    Main function for the "gulagcleaner" CLI command.

    The "gulagcleaner" command takes an argument for the path of a PDF file and tries to deembed the pages inside it. The pages are saved in a new PDF in the same folder.
    
    Available CLI arguments:
    -h : displays help information
    -r : replaces the original file with the deembedded file
    -o : uses the old deembeding method (for files older than 18/05/2023)
    -v : displays the version of the program

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
        print("  filename      the PDF file to deembed pages from.")
        print("")
        print("Optional arguments:")
        print("  -h            show this help message.")
        print("  -r            replaces the original file with the deembedded file.")
        print("  -o            uses the old deembeding method (for files older than 18/05/2023).")
        print("  -v            shows the version of the program.")
        return

    # Check for the -v argument
    if '-v' in sys.argv:
        print("actual version: 0.5.2")
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
    else:
        method = "new"
    
    # Call the deembed function
    return_msg = gulagcleaner_extract.deembed(filename, replace,method)
    if return_msg["Success"]:
        print("Deembedding successful. File saved in", return_msg["return_path"])
        print("Metadata:")
        print("Archivo: " + return_msg["Meta"]["Archivo"])
        print("Autor: " + return_msg["Meta"]["Autor"])
        print("Asignatura: " + return_msg["Meta"]["Asignatura"])
        print("Curso y Grado: " + return_msg["Meta"]["Curso y Grado"])
        print("Facultad: " + return_msg["Meta"]["Facultad"])
        print("Universidad: " + return_msg["Meta"]["Universidad"])
    else:
        print("Error:", return_msg["Error"])



if __name__ == "__main__":
    print('Call from the "gulagcleaner" command.')
