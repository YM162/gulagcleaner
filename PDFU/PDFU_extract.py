import os
from pdfrw import PdfReader, PdfWriter
from pdfrw.findobjs import wrap_object , find_objects
from pdfrw.objects import PdfName
from pikepdf import Pdf
import traceback
import tempfile

def clean_fun(pdf_path):
    prepdf=Pdf.open(pdf_path)
    prepdf.save(pdf_path[:-4]+"_inter.pdf")
    prepdf.close()

    pdf = PdfReader(pdf_path[:-4]+"_inter.pdf")
    xobjs=list(find_objects(pdf.pages,valid_subtypes=(PdfName.Form, PdfName.Ima)))
    páginas=[]
    for item in xobjs:
        páginas.append(wrap_object(item, 1000, 0.5*72))
    if len(xobjs)==0:
        os.remove(pdf_path[:-4]+"_inter.pdf")
        output=False
        return output
    del xobjs
    del pdf

    output=pdf_path[:-4]+"_limpio.pdf"
    writer = PdfWriter(output)
    writer.addpages(páginas)
    del páginas
    writer.write()
    del writer

    os.remove(pdf_path[:-4]+"_inter.pdf")

    return output


def clean():
    root = "C:/users/yomis/python/gulag_cleaner_standalone/"
    file="test.pdf"
    try:
        outputpath=clean_fun(root+file)
        print(outputpath)
    except Exception as e:
        print(e)
        traceback.print_exc()
if __name__ == "__main__":
    clean()