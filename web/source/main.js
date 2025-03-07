"use strict";

// @ts-check

import {
  htmlDocumentBody,
  htmlDocumentMetadataHeader,
  htmlDocumentRoot,
  htmlExternalResourceLink,
  htmlMain,
  htmlMetadata,
  htmlParagraph
} from "./html/create-element.js";

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  htmlDocumentRoot
  htmlDocumentRoot.lang = String("en");
  
  htmlDocumentMetadataHeader
  htmlExternalResourceLink
  htmlExternalResourceLink.href = String("/theme/dark.css");
  htmlExternalResourceLink.rel = String("stylesheet");
  htmlDocumentMetadataHeader.appendChild(htmlExternalResourceLink);
  htmlMetadata
  htmlMetadata.name = String("main-index");
  htmlMetadata.content = String("width=device-width");
  htmlDocumentMetadataHeader.appendChild(htmlMetadata);
  htmlDocumentRoot.appendChild(htmlDocumentMetadataHeader);
  
  htmlDocumentBody
  htmlDocumentBody.classList.add(String("dark"));
  htmlMain
  htmlParagraph
  htmlParagraph.innerText = String("Hyaena Technologies");
  htmlMain.appendChild(htmlParagraph);
  htmlDocumentBody.appendChild(htmlMain);
  htmlDocumentRoot.appendChild(htmlDocumentBody);
  
  return document.appendChild(htmlDocumentRoot);
}

main();

