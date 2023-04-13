import os
import pikepdf
from pdfminer.high_level import extract_text
from pdfrw import PdfReader, PdfWriter
from pdfrw.findobjs import wrap_object, find_objects
from pdfrw.objects import PdfName

def extract_metadata(pdf_path):
    """
    Extract metadata from a PDF file, including the author, subject, course and grade, faculty, and university.

    Args:
        pdf_path (str): The path to the pdf file.

    Returns:
        metadict (dict): A dictionary with the following keys and values:
            "Archivo": (str) File name.
            "Autor": (str) Author of the file.
            "Asignatura": (str) Subject.
            "Curso y Grado": (str) Course and degree.
            "Facultad": (str) Faculty.
            "Universidad": (str) University.
    """
    text = extract_text(pdf_path, maxpages=1)
    metalist = list(filter(None, text.splitlines()))
    return {
        "Archivo": metalist[0],
        "Autor": metalist[1],
        "Asignatura": metalist[2],
        "Curso y Grado": metalist[3],
        "Facultad": metalist[4],
        "Universidad": metalist[5]
    }

def deembed(pdf_path, replace=False):
    """
    De-embeds the PDF file and creates a new PDF file in the same folder with each embedded page in a new page.

    Args:
        pdf_path (str): The path to the PDF file.
        replace (bool, optional): If set to True, the original file will be replaced by the de-embedded file.
            Default is False.

    Returns:
        return_msg (dict): A dictionary with the following keys:
            success (bool): Indicates whether the de-embedding process was successful.
            return_path (str): The path to the de-embedded file if successful.
            error (str): An error description if the process was unsuccessful.
            meta (dict): Information about the file including "Archivo", "Autor", "Asignatura", "Curso y Grado", "Facultad", and "Universidad".
    """
    if not pdf_path.endswith(".pdf"):
        return {
            "Success": False,
            "return_path": "",
            "Error": "File is not a .pdf file.",
            "Meta": {}
        }

    intermediate_pdf_path = pdf_path[:-4] + "_inter.pdf"
    try:
        with pikepdf.Pdf.open(pdf_path) as pdf:
            pdf.save(intermediate_pdf_path)

        metadict = {}
        try:
            metadict = extract_metadata(pdf_path)
        except Exception as e:
            print("Failed to extract metadata:", e)

        pdf = PdfReader(intermediate_pdf_path)
        xobjs = list(find_objects(pdf.pages, valid_subtypes=(PdfName.Form, PdfName.Dummy)))
        pages = [wrap_object(item, 1000, 0.5 * 72) for item in xobjs]

        if not xobjs:
            os.remove(intermediate_pdf_path)
            return {
                "Success": False,
                "return_path": "",
                "Error": "No embedded pages found.",
                "Meta": metadict
            }

        output = pdf_path[:-4] + (".pdf" if replace else "_clean.pdf")
        writer = PdfWriter(output)
        writer.addpages(pages)
        writer.write()

        os.rename(output, output.replace('wuolah-free-', ''))
        os.remove(intermediate_pdf_path)

        return {
            "Success": True,
            "return_path": output,
            "Error": "",
            "Meta": metadict
        }
    except Exception as e:
        return {
            "Success": False,
            "return_path": "",
            "Error": str(e),
            "Meta": {}
        }
