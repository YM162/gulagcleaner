# Gulag-cleaner

Herramienta de eliminación de anuncios para PDFs.
Es un inverso funcional de las funciones que insertan páginas pdf dentro de otros documentos en forma de objetos de tipo Form. El ejemplo más prominente es la función embedPages() de la librería PDF-lib.js</br>

También es capaz de extraer los Metadatos (Autor, Asignatura, Universidad...) del archivo. Para más información consultar la descripción de la función.</br>


# Instalación</br>
>pip install gulagcleaner-xv</br>

# Opciones</br>
- -r : reemplaza los archivos originales
- -h : muestra información
- -v : muestra la versión actual
# Uso</br>
CLI:</br>
>gulagcleaner \<filename\></br>

Code:
>from gulagcleaner.gulagcleaner_extract import deembed
>
>return_msg = deembed( "file.pdf" , replace)
