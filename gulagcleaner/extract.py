from pdfrw import PdfReader, PdfWriter
from pdfrw.findobjs import wrap_object, find_objects
from pdfrw.objects import PdfName

def find_iobj_pairs(first_page, second_page):
    """
    Finds the indirect objects that repeat between pages.

    Args:
        first_page (PDFPage): The first page.
        second_page (PDFPage): The second page.
    
    Returns:
        return (tuple): (first_page,first_pair_index)
            first_pair_index (int): index of the first repeating indirect object on the first page.
    
    """
    comunes = tuple(set(first_page).intersection(second_page))
    if first_page.index(comunes[0]) < first_page.index(comunes[1]):
        return (first_page,first_page.index(comunes[0]),first_page.index(comunes[1]))
    else:
        return (first_page,first_page.index(comunes[1]),first_page.index(comunes[0]))
    
def clean_pdf(pdf_path, output_path="", method="new"):
    """
    De-embeds the PDF file and creates a new PDF file in the same folder with each embedded page in a new page.

    Args:
        pdf_path (str): The path to the PDF file.
        output_path (str): The path to the output PDF file.
        method (str, optional): Defines what strategy will be used to clean the pdf file. Can be "new", "old" or "naive".
            Default is "new".

    Returns:
        return_msg (dict): A dictionary with the following keys:
            success (bool): Indicates whether the de-embedding process was successful.
            return_path (str): The path to the de-embedded file if successful.
            error (str): An error description if the process was unsuccessful.
    """

    if not pdf_path.endswith(".pdf"):
        return {
            "Success": False,
            "return_path": "",
            "Error": "File is not a .pdf file.",
        }

    if output_path == "":
        output_path = pdf_path[:-4] + "_clean.pdf"

    try:
        pdf = PdfReader(pdf_path)

        # Old method, for files older than 18/05/2023. Works by finding the embedded pages and creating new pages with them.
        if method=="old":
            xobjs = []
            for page in pdf.pages:
                xobjs.extend([page.Resources.XObject[object] for object in page.Resources.XObject if "EmbeddedPdfPage" in str(object)])
            newpages = [wrap_object(item, 1000, 0.5 * 72) for item in xobjs]

            if not xobjs:
                return {
                    "Success": False,
                    "return_path": "",
                    "Error": "No embedded pages found."
                }
            
        # New method, for files newer than 18/05/2023. Works by finding the repeating indirect objects between pages and creating new pages with them.
        elif method=="new":
            content_list = []
            for page in pdf.pages:
                content = [content.indirect for content in page.Contents]
                if len(content)>1:
                    content_list.append(content)

            new_contents = []
            for i in range(0,len(content_list)):
                if i == len(content_list)-1:
                    pares = find_iobj_pairs(content_list[i], content_list[i-1])
                else:
                    pares = find_iobj_pairs(content_list[i], content_list[i + 1])
                new_contents.append(pares[0][pares[1]-2:pares[2]+4])

            newpages = []
            for i,page in enumerate([page for page in pdf.pages if len(page.Contents)>1]):
                newpage = page.copy()
                newpage.Contents = [pdf.indirect_objects[iobj] for iobj in new_contents[i]]
                newpage.Annots = []
                newpages.append(newpage)

        #Naive method. Just detects the pages with ads and crops them. THIS METHOD IS NOT RECOMENDED AT ALL. It is very unreliable and when copying text from the outputed pdf the ads and watermaks are copied as well, because we are just "hiding" them from the user, not truly removing them.
        elif method=="naive":
            #Not yet implemented.
            newpages = []

        else:
            return {
                "Success": False,
                "return_path": "",
                "Error": "Cleaning method not found."
            }
        
        writer = PdfWriter(output_path)
        writer.addpages(newpages)
        writer.write()

        return {
            "Success": True,
            "return_path": output_path,
            "Error": ""
            }
    
    except Exception as e:
        return {
            "Success": False,
            "return_path": "",
            "Error": str(e),
            "Meta": {}
        }
