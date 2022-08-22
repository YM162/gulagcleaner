# Gulag-cleaner-cli

Herramienta de eliminación de anuncios en PDFs generados por la plataforma Wuolah.
Es un inverso funcional de las funciones que insertan páginas pdf dentro de otros documentos. El ejemplo más prominente es la función embedPages() de la librería PDF-lib.js</br>

Adicionalmente también es capaz de extraer los Metadatos (Autor, Asignatura, Universidad...) del archivo. Para más información consultar la descripción de la función.</br>

# Como instalar</br>
>pip install gulagcleaner</br>

# Uso</br>
CLI:</br>
>gulagcleaner \<filename\></br>

Code:
>from gulagcleaner.gulagcleaner_extract import deembed
>
>return_msg = deembed( "file.pdf" )