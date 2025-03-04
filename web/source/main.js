"use strict";

// @ts-check

import "./html/create-element.js"

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  HTMLDocumentRoot
  HTMLDocumentRoot.lang = String("en");
  
  HTMLDocumentMetadataHeader
  HTMLExternalResourceLink
  HTMLExternalResourceLink.href = String("/theme/dark.css");
  HTMLExternalResourceLink.rel = String("stylesheet");
  HTMLDocumentMetadataHeader.appendChild(HTMLExternalResourceLink);
  HTMLMetadata
  HTMLMetadata.name = String("main-index");
  HTMLMetadata.content = String("width=device-width");
  HTMLDocumentMetadataHeader.appendChild(HTMLMetadata);
  HTMLDocumentRoot.appendChild(HTMLDocumentMetadataHeader);
  
  HTMLDocumentBody
  HTMLDocumentBody.classList.add(String("dark"));
  HTMLMain
  HTMLParagraph
  HTMLParagraph.innerText = String("Hyaena Technologies");
  HTMLMain.appendChild(HTMLParagraph);
  HTMLDocumentBody.appendChild(HTMLMain);
  HTMLDocumentRoot.appendChild(HTMLDocumentBody);
  
  return document.body.appendChild(HTMLDocumentRoot);
}

main();

