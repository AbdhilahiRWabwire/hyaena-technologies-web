"use strict";

// @ts-check

export {
    HTTP_CONNECT,
    HTTP_DELETE,
    HTTP_GET,
    HTTP_HEAD,
    HTTP_OPTIONS,
    HTTP_PATCH,
    HTTP_POST,
    HTTP_PUT,
    HTTP_TRACE,
    httpMethods
}

// Hypertext Transfer Protocol Methods
/** @type {string} */
const HTTP_CONNECT = String("CONNECT");
/** @type {string} */
const HTTP_DELETE = String("DELETE");
/** @type {string} */
const HTTP_GET = String("GET");
/** @type {string} */
const HTTP_HEAD = String("HEAD");
/** @type {string} */
const HTTP_OPTIONS = String("OPTIONS");
/** @type {string} */
const HTTP_PATCH = String("PATCH");
/** @type {string} */
const HTTP_POST = String("POST");
/** @type {string} */
const HTTP_PUT = String("PUT");
/** @type {string} */
const HTTP_TRACE = String("TRACE");

// Hypertext Transfer Protocol Method Vector
/** @type {function(): string[][]} */
function httpMethods() {
    const hypertextTransferMethods = new Array([
        HTTP_CONNECT,
        HTTP_DELETE,
        HTTP_GET,
        HTTP_HEAD,
        HTTP_OPTIONS,
        HTTP_PATCH,
        HTTP_POST,
        HTTP_PUT,
        HTTP_TRACE
    ]);

    return hypertextTransferMethods;
}
