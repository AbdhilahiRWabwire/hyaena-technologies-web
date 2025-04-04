"use strict";

// @ts-check

export {
    httpMethods
}

// Hypertext Transfer Protocol Methods
/** @type {function(): string[][]} */
function httpMethods() {
    const hypertextMethods = new Array([
        "CONNECT",
        "DELETE",
        "GET",
        "HEAD",
        "OPTIONS",
        "PATCH",
        "POST",
        "PUT",
        "TRACE"
    ]);

    return hypertextMethods;
}
