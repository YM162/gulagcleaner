from gulagcleaner.extract import clean_pdf
from gulagcleaner.decrypt import decrypt_pdf
from gulagcleaner.metadata import extract_metadata
from os.path import exists

def main():
    '''
    Main function for the "gulagcleaner" CLI command.

    The "gulagcleaner" command takes an argument for the path of a PDF file and tries to remove the ads inside it. The new PDF is saved in the same folder.
    
    Available CLI arguments:
    -h : Display help information.
    -r : Replace the original file with the cleaned file.
    -o : Use the old cleaning method (for files older than 18/05/2023).
    -v : Display the version of the program.

    '''
    import sys
    import os.path

    # Check for the -h argument
    if '-h' in sys.argv:
        print("Usage: gulagcleaner [-h] [-r] [-o] [-v] <pdf_path>")
        print("")
        print("Removes ads from a PDF file.")
        print("")
        print("Positional arguments:")
        print("  pdf_path      The PDF file to clean.")
        print("")
        print("Optional arguments:")
        print("  -h            Show this help message.")
        print("  -r            Replace the original file with the cleaned file.")
        print("  -o            Use the old cleaning method (for files older than 18/05/2023).")
        print("  -v            Show the version of the program.")
        return

    # Check for the -v argument
    if '-v' in sys.argv:
        print("Current version: 0.6.4")
        return

    # Get the pdf_path argument
    if len(sys.argv) < 2:
        print('Usage: gulagcleaner [-h] [-r] [-o] [-v] <pdf_path>')
        return
    pdf_path = sys.argv[-1]

    # Check if the file exists
    if not exists(pdf_path):
        print("File not found.")
        return

    # Check if the -r argument is present
    if '-r' in sys.argv:
        output_path = pdf_path
    else:
        output_path = pdf_path[:-4] + "_clean.pdf"

     # Check if the -o argument is present
    if '-o' in sys.argv:
        method = "old"
        pdf_path = decrypt_pdf(pdf_path)
        intermediate = True
    else:
        method = "new"
        intermediate = False

    #Extract metadata
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
    return_msg = clean_pdf(pdf_path, output_path, method)

    if return_msg["Success"]:
        print("Cleaning successful. File saved in", return_msg["return_path"])
    else:
        print("Error:", return_msg["Error"])

    if intermediate:
        os.remove(pdf_path)

if __name__ == "__main__":
    print('Call from the "gulagcleaner" command.')
