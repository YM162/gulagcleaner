# PDFU
PDF Unembedder: Functional inverse of functions that embbed pdf pages inside other documents. The most prominent example is the embedPages() function of PDF-lib.js</br>

This has the side efect of removing ads and watermarks placed by many websites.

# How to install:</br>
>pip install pdfu</br>

# Usage</br>
CLI:</br>
>pdfu \<filename\></br>

Code:
>from PDFU.PDFU_Extract import deembed
>
>return_msg = deembed( "file.pdf" )
