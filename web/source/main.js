"use strict";

// @ts-check

import "/utility/create-element.js"

// Main Entry Point
/** @type {function(): void} */
function main() {
  /** @type {HTMLDivElement} */
  const division = document.createElement('div');

  document.body.appendChild(division);
}
