"use strict";

// @ts-check

import "./html/create-element.js"

// Main Entry Point
/** @type {function(): void} */
function main() {
  HTMLDocumentRoot
  HTMLDocumentRoot.lang = "en";
  
  documentMetadataHeader
  HTMLMetadata
  HTMLMetadata.name = "main-index";
  HTMLMetadata.content = "width=device-width";
  documentMetadataHeader.appendChild(HTMLMetadata);
  HTMLDocumentRoot.appendChild(documentMetadataHeader);
  
  documentBody
  HTMLMain
  HTMLParagraph
  HTMLParagraph.innerText = "Hyaena Technologies";
  HTMLMain.appendChild(HTMLParagraph);
  documentBody.appendChild(HTMLMain);
  HTMLDocumentRoot.appendChild(documentBody);
  
  document.body.style.background = 'lightblue';
  document.body.appendChild(HTMLDocumentRoot);
}

main();
