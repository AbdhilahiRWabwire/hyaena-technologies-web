"use strict";

// @ts-check

export {
    HTTP_VERSION_ONE,
    HTTP_VERSION_TWO,
    HTTP_VERSION_THREE,
    httpVersions
}

// Hypertext Transfer Protocol Versions
/** @type {string} */
const HTTP_VERSION_ONE: HTTPVersion = String("HTTP/1.1");
/** @type {string} */
const HTTP_VERSION_TWO: HTTPVersion = String("HTTP/2.0");
/** @type {string} */
const HTTP_VERSION_THREE: HTTPVersion = String("HTTP/3.0");

// Hypertext Transfer Protocol Version Vector
/** @type {function(): string[][]} */
function httpVersions() {
    const hypertextTransferVersions = new Array([        
        HTTP_VERSION_ONE,
        HTTP_VERSION_TWO,
        HTTP_VERSION_THREE
    ]);

    return hypertextTransferVersions;
}
