# Gulag-cleaner

Herramienta de eliminación de anuncios para PDFs.
Es un inverso funcional de las funciones que insertan páginas pdf dentro de otros documentos en forma de Form XObjects (). El ejemplo que más nos interesa es la función embedPages() de la librería pdf-lib.js</br>

También es capaz de extraer los Metadatos (Autor, Asignatura, Universidad...) del archivo. Para más información consultar la descripción de la función.</br>


# Instalación</br>
>pip install gulagcleaner</br>

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
