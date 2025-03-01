"use strict";

// @ts-check

import "./html/create-element.js"

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  HTMLDocumentRoot
  HTMLDocumentRoot.lang = "en";
  
  documentMetadataHeader
  externalResourceLink
  externalResourceLink.href = "theme/dark.css";
  externalResourceLink.rel = "stylesheet";
  documentMetadataHeader.appendChild(externalResourceLink);
  HTMLMetadata
  HTMLMetadata.name = "main-index";
  HTMLMetadata.content = "width=device-width";
  documentMetadataHeader.appendChild(HTMLMetadata);
  HTMLDocumentRoot.appendChild(documentMetadataHeader);
  
  documentBody
  documentBody.classList.add("dark");
  HTMLMain
  HTMLParagraph
  HTMLParagraph.innerText = "Hyaena Technologies";
  HTMLMain.appendChild(HTMLParagraph);
  documentBody.appendChild(HTMLMain);
  HTMLDocumentRoot.appendChild(documentBody);
  
  return document.body.appendChild(HTMLDocumentRoot);
}

main();
