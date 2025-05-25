"use strict"

// @ts-check

export {
    HypertextMessage
}

// Hypertext Transfer Protocol Message Definition
/** @type {HypertextMessage} */
class HypertextTransferMessage {
    constructor(
        /** @template T */
        /** @type {T} */
        body,
        /** @type {string[][]} */
        headers,
        /** @type {string} */
        method,
        /** @type {string[][]} */
        security_directives,
        /** @type {string} */
        status_code,
        /** @type {string} */
        status_text,
        /** @type {string} */
        version
    ) {
         /** @template T */
        /** @type {T} */
        this.body = body;
        /** @type {string[][]} */
        this.headers = headers;
        /** @type {string} */
        this.method = method;
        /** @type {string[][]} */
        this.security_directives = security_directives;
        /** @type {string} */
        this.status_code = status_code;
        /** @type {string} */
        this.status_text = status_text;
        /** @type {string} */
        this.version = version;
      }
}
