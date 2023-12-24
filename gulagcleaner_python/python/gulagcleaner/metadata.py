from pdfminer.high_level import extract_text

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