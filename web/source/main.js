"use strict";

// @ts-check

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  /** @type {HTMLBodyElement} */
  const htmlDocumentBody = document.createElement("body");
  /** @type {HTMLHeadElement} */
  const htmlDocumentMetadataHeader = document.createElement("head");
  /** @type {HTMLHtmlElement} */
  const htmlDocumentRoot = document.createElement("html");
  /** @type {HTMLLinkElement} */
  const htmlExternalResourceLink = document.createElement("link");
  /** @type {HTMLElement} */
  const htmlMain = document.createElement("main");
  /** @type {HTMLMetaElement} */
  const htmlMetadata = document.createElement("meta");
  /** @type {HTMLParagraphElement} */
  const htmlParagraph = document.createElement("p");

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

