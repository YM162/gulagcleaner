import os
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
        return (first_page,first_page.index(comunes[0]))
    else:
        return (first_page,first_page.index(comunes[1]))
    
def deembed(pdf_path, replace=False, method="new"):
    """
    De-embeds the PDF file and creates a new PDF file in the same folder with each embedded page in a new page.

    Args:
        pdf_path (str): The path to the PDF file.
        replace (bool, optional): If set to True, the original file will be replaced by the de-embedded file.
            Default is False.
        method (str, optional): Defines what strategy will be used to deembed the pdf file. Can be "old" or "new".
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

    try:
        pdf = PdfReader(pdf_path)

        # Old method, for files older than 18/05/2023. Works by finding the embedded pages and creating new pages with them.
        if method=="old":
            xobjs = list(find_objects(pdf.pages, valid_subtypes=(PdfName.Form, PdfName.Dummy)))
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
                new_contents.append(pares[0][pares[1]-2:pares[1]+6])

            newpages = []
            for i,page in enumerate([page for page in pdf.pages if len(page.Contents)>1]):
                newpage = page.copy()
                newpage.Contents = [pdf.indirect_objects[iobj] for iobj in new_contents[i]]
                newpage.Annots = []
                newpages.append(newpage)

        else:
            return {
                "Success": False,
                "return_path": "",
                "Error": "Deembeding method not found."
            }
        
        output = pdf_path[:-4] + (".pdf" if replace else "_clean.pdf")
        writer = PdfWriter(output)
        writer.addpages(newpages)
        writer.write()

        return {
            "Success": True,
            "return_path": output,
            "Error": ""
            }
    
    except Exception as e:
        return {
            "Success": False,
            "return_path": "",
            "Error": str(e),
            "Meta": {}
        }
