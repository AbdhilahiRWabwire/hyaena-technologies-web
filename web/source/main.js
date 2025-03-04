"use strict";

// @ts-check

import "./html/create-element.js"

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  HTMLDocumentRoot
  HTMLDocumentRoot.lang = String("en");
  
  documentMetadataHeader
  externalResourceLink
  externalResourceLink.href = String("/theme/dark.css");
  externalResourceLink.rel = String("stylesheet");
  documentMetadataHeader.appendChild(externalResourceLink);
  HTMLMetadata
  HTMLMetadata.name = String("main-index");
  HTMLMetadata.content = String("width=device-width");
  documentMetadataHeader.appendChild(HTMLMetadata);
  HTMLDocumentRoot.appendChild(documentMetadataHeader);
  
  documentBody
  documentBody.classList.add(String("dark"));
  HTMLMain
  HTMLParagraph
  HTMLParagraph.innerText = String("Hyaena Technologie");
  HTMLMain.appendChild(HTMLParagraph);
  documentBody.appendChild(HTMLMain);
  HTMLDocumentRoot.appendChild(documentBody);
  
  return document.body.appendChild(HTMLDocumentRoot);
}

main();
