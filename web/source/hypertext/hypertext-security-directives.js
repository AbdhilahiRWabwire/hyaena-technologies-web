"use strict";

// @ts-check

export {
    httpSecurityDirectives
}

// Hypertext Transfer Protocol Content Security Policy Directives
/** @type {function(): string[][]} */
function httpSecurityDirectives() {
    const hypertextSecurityDirectives = new Array([
        "base-uri",
        "child-src",
        "connect-src",
        "default-src",
        "font-src",
        "form-action",
        "frame-ancestors",
        "frame-src",
        "img-src",
        "manifest-src",
        "media-src",
        "object-src",
        "report-to",
        "require-trusted-types-for",
        "sandbox",
        "script-src",
        "script-src-attr",
        "script-src-elem",
        "style-src",
        "style-src-attr",
        "style-src-elem",
        "trusted-types",
        "upgrade-insecure-requests",
        "worker-src"
    ]);

    return hypertextSecurityDirectives;
}
