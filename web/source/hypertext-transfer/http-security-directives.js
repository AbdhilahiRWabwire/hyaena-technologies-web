"use strict"

// @ts-check

export {
    HTTP_BASE_URI,
    HTTP_CHILD_SOURCE,
    HTTP_CONNECT_SOURCE,
    HTTP_DEFAULT_SOURCE,
    HTTP_FONT_SOURCE,
    HTTP_FORM_ACTION,
    HTTP_FRAME_ANCESTORS,
    HTTP_FRAME_SOURCE,
    HTTP_IMAGE_SOURCE,
    HTTP_MANIFEST_SOURCE,
    HTTP_MEDIA_SOURCE,
    HTTP_OBJECT_SOURCE,
    HTTP_REPORT_TO,
    HTTP_REQUIRE_TRUSTED_TYPES_FOR,
    HTTP_SANDBOX,
    HTTP_SCRIPT_SOURCE,
    HTTP_SCRIPT_SOURCE_ATTRIBUTE,
    HTTP_SCRIPT_SOURCE_ELEMENT,
    HTTP_STYLE_SOURCE,
    HTTP_STYLE_SOURCE_ATTRIBUTE,
    HTTP_STYLE_SOURCE_ELEMENT,
    HTTP_TRUSTED_TYPES,
    HTTP_UPGRADE_INSECURE_REQUESTS,
    HTTP_WORKER_SOURCE,
    httpSecurityDirectives
}

// Hypertext Transfer Protocol Content Security Policy Directives
/** @type {string} */
const HTTP_BASE_URI = String("base-uri");
/** @type {string} */
const HTTP_CHILD_SOURCE = String("child-src");
/** @type {string} */
const HTTP_CONNECT_SOURCE = String("connect-src");
/** @type {string} */
const HTTP_DEFAULT_SOURCE = String("default-src");
/** @type {string} */
const HTTP_FONT_SOURCE = String("font-src");
/** @type {string} */
const HTTP_FORM_ACTION = String("form-action");
/** @type {string} */
const HTTP_FRAME_ANCESTORS = String("frame-ancestors");
/** @type {string} */
const HTTP_FRAME_SOURCE = String("frame-src");
/** @type {string} */
const HTTP_IMAGE_SOURCE = String("img-src");
/** @type {string} */
const HTTP_MANIFEST_SOURCE = String("manifest-src");
/** @type {string} */
const HTTP_MEDIA_SOURCE = String("media-src");
/** @type {string} */
const HTTP_OBJECT_SOURCE = String("object-src");
/** @type {string} */
const HTTP_REPORT_TO = String("report-to");
/** @type {string} */
const HTTP_REQUIRE_TRUSTED_TYPES_FOR = String("require-trusted-types-for");
/** @type {string} */
const HTTP_SANDBOX = String("sandbox");
/** @type {string} */
const HTTP_SCRIPT_SOURCE = String("script-src");
/** @type {string} */
const HTTP_SCRIPT_SOURCE_ATTRIBUTE = String("script-src-attr");
/** @type {string} */
const HTTP_SCRIPT_SOURCE_ELEMENT = String("script-src-elem");
/** @type {string} */
const HTTP_STYLE_SOURCE = String("style-src");
/** @type {string} */
const HTTP_STYLE_SOURCE_ATTRIBUTE = String("style-src-attr");
/** @type {string} */
const HTTP_STYLE_SOURCE_ELEMENT = String("style-src-elem");
/** @type {string} */
const HTTP_TRUSTED_TYPES = String("trusted-types");
/** @type {string} */
const HTTP_UPGRADE_INSECURE_REQUESTS = String("upgrade-insecure-requests");
/** @type {string} */
const HTTP_WORKER_SOURCE = String("worker-src");

// Hypertext Transfer Protocol Content Security Policy Directive Vector
/** @type {function(): string[][]} */
function httpSecurityDirectives() {
    const hypertextTransferSecurityDirectives = new Array([
        HTTP_BASE_URI,
        HTTP_CHILD_SOURCE,
        HTTP_CONNECT_SOURCE,
        HTTP_DEFAULT_SOURCE,
        HTTP_FONT_SOURCE,
        HTTP_FORM_ACTION,
        HTTP_FRAME_ANCESTORS,
        HTTP_FRAME_SOURCE,
        HTTP_IMAGE_SOURCE,
        HTTP_MANIFEST_SOURCE,
        HTTP_MEDIA_SOURCE,
        HTTP_OBJECT_SOURCE,
        HTTP_REPORT_TO,
        HTTP_REQUIRE_TRUSTED_TYPES_FOR,
        HTTP_SANDBOX,
        HTTP_SCRIPT_SOURCE,
        HTTP_SCRIPT_SOURCE_ATTRIBUTE,
        HTTP_SCRIPT_SOURCE_ELEMENT,
        HTTP_STYLE_SOURCE,
        HTTP_STYLE_SOURCE_ATTRIBUTE,
        HTTP_STYLE_SOURCE_ELEMENT,
        HTTP_TRUSTED_TYPES,
        HTTP_UPGRADE_INSECURE_REQUESTS,
        HTTP_WORKER_SOURCE
    ]);
    
    return hypertextTransferSecurityDirectives;
}
