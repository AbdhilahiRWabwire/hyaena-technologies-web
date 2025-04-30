"use strict";

// @ts-check

// Main Entry Point
/** @type {function(): HTMLHtmlElement} */
function main() {
  /** @type {HTMLBodyElement} */
  const HTML_DOCUMENT_BODY = document.createElement("body");
  /** @type {HTMLHeadElement} */
  const HTML_DOCUMENT_METADATA_HEADER = document.createElement("head");
  /** @type {HTMLHtmlElement} */
  const HTML_DOCUMENT_ROOT = document.createElement("html");
  /** @type {HTMLLinkElement} */
  const HTML_EXTERNAL_RESOURCE_LINK = document.createElement("link");
  /** @type {HTMLElement} */
  const HTML_MAIN = document.createElement("main");
  /** @type {HTMLMetaElement} */
  const HTML_METADATA = document.createElement("meta");
  /** @type {HTMLParagraphElement} */
  const HTML_PARAGRAPH = document.createElement("p");
  
  HTML_DOCUMENT_ROOT
  HTML_DOCUMENT_ROOT.lang = String("en");
  
  HTML_DOCUMENT_METADATA_HEADER
  HTML_EXTERNAL_RESOURCE_LINK
  HTML_EXTERNAL_RESOURCE_LINK.href = String("/theme/dark.css");
  HTML_EXTERNAL_RESOURCE_LINK.rel = String("stylesheet");
  HTML_DOCUMENT_METADATA_HEADER.appendChild(HTML_EXTERNAL_RESOURCE_LINK);
  HTML_METADATA
  HTML_METADATA.name = String("main-index");
  HTML_METADATA.content = String("width=device-width");
  HTML_DOCUMENT_METADATA_HEADER.appendChild(HTML_METADATA);
  HTML_DOCUMENT_ROOT.appendChild(HTML_DOCUMENT_METADATA_HEADER);
  
  HTML_DOCUMENT_BODY
  HTML_DOCUMENT_BODY.classList.add(String("dark"));
  HTML_MAIN
  HTML_PARAGRAPH
  HTML_PARAGRAPH.innerText = String("Hyaena Technologies");
  HTML_MAIN.appendChild(HTML_PARAGRAPH);
  HTML_DOCUMENT_BODY.appendChild(HTML_MAIN);
  HTML_DOCUMENT_ROOT.appendChild(HTML_DOCUMENT_BODY);
  
  return document.appendChild(HTML_DOCUMENT_ROOT);
}

main();

