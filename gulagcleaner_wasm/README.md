# Gulag Cleaner WASM Distribution
![jsDelivr hits (npm)](https://img.shields.io/jsdelivr/npm/hm/gulagcleaner_wasm)

Below is a minimal example showcasing the use of `gulagcleaner_wasm`, using raw HTML and JavaScript. It runs serverless, static, and client-side.

## index.html

```html
<script type="module">
  import {
    initWasmModule,
    cleanPdfBytes,
    bundleAndDownload
  } from './main.js';

  // Example: wire up a file input and a button
  const input = document.querySelector('#pdfInput');
  const button = document.querySelector('#cleanBtn');

  button.addEventListener('click', async () => {
    await initWasmModule();            // 1) load the WASM
    const results = [];

    for (const file of input.files) { // 2) process each PDF
      const arr     = new Uint8Array(await file.arrayBuffer());
      const cleaned = await cleanPdfBytes(arr);
      results.push({ name: file.name, data: cleaned });
    }

    bundleAndDownload(results);        // 3) download cleaned PDFs
  });
</script>

<input id="pdfInput" type="file" accept=".pdf" multiple>
<button id="cleanBtn">Clean & Download</button>

```


## main.js

```js
// 1) Default import is the async initializer ('init'), 
//    and named export 'clean_pdf' is the actual cleaning function.
import initWasm, { clean_pdf as wasmCleanPdf }
  from 'https://cdn.jsdelivr.net/npm/gulagcleaner_wasm/gulagcleaner_wasm.js';

let wasmReady = false;

/**
 * Initialize the WebAssembly module.
 * @param {string} [wasmUrl] Optional override for the .wasm file URL.
 * @returns {Promise<void>}
 */
export async function initWasmModule(wasmUrl) {
  // The initializer accepts either undefined or a URL
  await initWasm(wasmUrl);
  wasmReady = true;
}

/**
 * Cleans a single PDF.
 * @param {Uint8Array} pdfBytes – raw PDF data
 * @returns {Promise<Uint8Array>} cleaned PDF data
 */
export async function cleanPdfBytes(pdfBytes) {
  if (!wasmReady) {
    throw new Error('WASM not initialized. Call initWasmModule() first.');
  }

  // try default method (force_naive = false)
  try {
    return wasmCleanPdf(pdfBytes, 0);
  } catch (_) {
    // fallback to naive method (force_naive = true)
    return wasmCleanPdf(pdfBytes, 1);
  }
}

/**
 * Bundles one or multiple results into a download.
 * @param {Array<{name: string, data: Uint8Array|Blob}>} results
 * @param {string} [zipFilename='cleaned-pdfs.zip']
 */
export async function bundleAndDownload(results, zipFilename = 'cleaned-pdfs.zip') {
  if (results.length === 0) {
    console.warn('No PDFs to download.');
    return;
  }

  // Single file → direct download
  if (results.length === 1) {
    const { name, data } = results[0];
    const blob = data instanceof Blob
      ? data
      : new Blob([data], { type: 'application/pdf' });
    triggerDownload(blob, name.replace(/^wuolah-(free|premium)-/, '').replace(/\.pdf$/i, '-gulag-free.pdf'));
    return;
  }

  // Multiple files → ZIP download via JSZip
  let JSZip = window.JSZip;
  if (!JSZip) {
    // dynamic ESM import of JSZip
    JSZip = (await import('https://cdnjs.cloudflare.com/ajax/libs/jszip/3.10.0/jszip.min.js'))
              .default;
  }
  const zip = new JSZip();
  for (const { name, data } of results) {
    zip.file(name.replace(/^wuolah-(free|premium)-/, '').replace(/\.pdf$/i, '-gulag-free.pdf'), data);
  }
  const zipBlob = await zip.generateAsync({ type: 'blob' });
  triggerDownload(zipBlob, zipFilename);
}

/** Helper to trigger a download of a Blob */
function triggerDownload(blob, filename) {
  const url = URL.createObjectURL(blob);
  const a   = document.createElement('a');
  a.href    = url;
  a.download= filename;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1e3);
}
```