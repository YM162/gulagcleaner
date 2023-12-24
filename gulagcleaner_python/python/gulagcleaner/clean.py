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
            error (str): An error description if the process was unsuccessful.
    """
    try:
        with open(pdf_path, "rb") as f:
            pdf = f.read()
            cleaned_pdf = clean_pdf(pdf, force_naive)
            with open(output_path, "wb") as f:
                f.write(bytes(cleaned_pdf))
            return {"success": True, 
                    "return_path": output_path, 
                    "error": ""}
    except Exception as e:
        return {"success": False, "return_path": "", "error": str(e)}
    
# I would like to expose the clean_pdf function as a "clean_pdf_bytes(bytes)" function that already removes the trailing method byte and returns the cleaned bytes and the method. 
# Working on it.