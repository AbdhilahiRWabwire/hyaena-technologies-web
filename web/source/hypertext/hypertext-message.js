"use strict";

// @ts-check

// Hypertext Transfer Protocol Message Definition
/** @type {HypertextMessage} */
class HypertextMessage {
    constructor(
        /** @template T */
        /** @type {T} */
        body,
        /** @type {string[][]} */
        headers,
        /** @type {string} */
        method,
        /** @type {string} */
        statusCode,
        /** @type {string} */
        statusText,
        /** @type {string} */
        version,

    ) {
        /** @type {T} */
        this.body = body;
        /** @type {string[][]} */
        this.headers = headers;
        /** @type {string} */
        this.method = method;
        /** @type {string} */
        this.statusCode = statusCode;
        /** @type {string} */
        this.statusText = statusText;
        /** @type {string} */
        this.version = version;
    }
}