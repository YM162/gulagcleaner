from ._lib import clean_pdf  # export public parts of the binary extension

def clean_pdf_path(pdf_path, output_path, force_naive=False):
    """
    Cleans the ads from the PDF file in a given path and saves it in another path.
    Args:
        pdf_path (str): The path to the pdf file.
        output_path (str): The path to save the cleaned pdf file.
        force_naive (bool): Whether to force the naive cleaning method.
    Returns:
        return_msg (dict): A dictionary with the following keys:
            success (bool): Indicates whether the de-embedding process was successful.
            return_path (str): The path to the cleaned file if successful.
            method (int): The numerical code of the method used to clean the file (0-Wuolah,1-StuDocu,2-Naive).
            error (str): An error description if the process was unsuccessful.
    """
    try:
        with open(pdf_path, "rb") as f:
            pdf = f.read()
            cleaned_pdf, method = clean_pdf(pdf, force_naive)
            with open(output_path, "wb") as f:
                f.write(bytes(cleaned_pdf))
            return {"success": True, 
                    "return_path": output_path, 
                    "method": method,
                    "error": ""}
    except Exception as e:
        return {"success": False, "return_path": "", "method":"","error": str(e)}
    
def clean_pdf_bytes(pdf_bytes, force_naive=False):
    """
    Cleans the ads from a PDF file given as bytes.
    Args:
        pdf_bytes (bytes): The bytes of the pdf file.
        force_naive (bool): Whether to force the naive cleaning method.
    Returns:
        return_msg (dict): A dictionary with the following keys:
            success (bool): Indicates whether the de-embedding process was successful.
            return_bytes (bytes): The bytes of the cleaned file if successful.
            method (int): The numerical code of the method used to clean the file (0-Wuolah,1-StuDocu,2-Naive).
            error (str): An error description if the process was unsuccessful.
    """
    try:
        cleaned_pdf, method = clean_pdf(pdf_bytes, force_naive)
        return {"success": True, 
                "return_bytes": bytes(cleaned_pdf), 
                "method": method,
                "error": ""}
    except Exception as e:
        return {"success": False, "return_path": "", "method":"","error": str(e)}