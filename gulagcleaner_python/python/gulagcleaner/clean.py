from ._lib import clean_pdf  # export public parts of the binary extension

#Here there should only be a function clean_pdf(pdf_path, output_path, force_naive) 
#that calls the rust function and then saves the pdf in the given output_path.
#It should return a dictionary with the following keys:
#     Returns:
#         return_msg (dict): A dictionary with the following keys:
#             success (bool): Indicates whether the de-embedding process was successful.
#             return_path (str): The path to the de-embedded file if successful.
#             error (str): An error description if the process was unsuccessful.
#     """

def clean_pdf(pdf_path, output_path, force_naive):
    return clean_pdf(10,6)