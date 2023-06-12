import pikepdf

def decrypt_pdf(pdf_path):
    intermediate_pdf_path = pdf_path[:-4] + "_inter.pdf"
    with pikepdf.Pdf.open(pdf_path) as pdf:
            pdf.save(intermediate_pdf_path)
    return intermediate_pdf_path
