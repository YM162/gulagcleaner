import pikepdf

def decrypt_pdf(pdf_path):
    """
    Decrypts a PDF file and returns the path to the decrypted file.
    Args:
        pdf_path (str): The path to the pdf file.
    Returns:
        intermediate_pdf_path (str): The path to the decrypted pdf file.
    """
    intermediate_pdf_path = pdf_path[:-4] + "_inter.pdf"
    with pikepdf.Pdf.open(pdf_path) as pdf:
            pdf.save(intermediate_pdf_path)
    return intermediate_pdf_path
