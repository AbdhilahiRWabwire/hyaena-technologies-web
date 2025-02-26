"use strict";

// @ts-check

import "./html/create-element.js"

// Main Entry Point
/** @type {function(): void} */
function main() {
  HTMLDocumentRoot

  HTMLDocumentRoot.lang = "en";

  documentMetadataHeader

  externalResourceLink

  externalResourceLink.href = "theme.css";

  externalResourceLink.rel = "stylesheet";

  externalResourceLink.type = "text/css";

  documentMetadataHeader.appendChild(externalResourceLink);

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

  document.body.appendChild(HTMLDocumentRoot);
}
