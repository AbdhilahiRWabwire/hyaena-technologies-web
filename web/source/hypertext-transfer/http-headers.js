"use strict"

// @ts-check

export {
    HTTP_ACCEPT,
    HTTP_ACCEPT_CLIENT_HINT,
    HTTP_ACCEPT_ENCODING,
    HTTP_ACCEPT_LANGUAGE,
    HTTP_ACCEPT_PATCH,
    HTTP_ACCEPT_POST,
    HTTP_ACCEPT_RANGES,
    HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS,
    HTTP_ACCESS_CONTROL_ALLOW_HEADERS,
    HTTP_ACCESS_CONTROL_ALLOW_METHODS,
    HTTP_ACCESS_CONTROL_ALLOW_ORIGIN,
    HTTP_ACCESS_CONTROL_EXPOSE_HEADERS,
    HTTP_ACCESS_CONTROL_MAX_AGE,
    HTTP_ACCESS_CONTROL_REQUEST_HEADERS,
    HTTP_ACCESS_CONTROL_REQUEST_METHOD,
    HTTP_AGE,
    HTTP_ALLOW,
    HTTP_ALTERNATIVE_SERVICE,
    HTTP_ALTERNATIVE_USED,
    HTTP_AUTHORIZATION,
    HTTP_CACHE_CONTROL,
    HTTP_CLEAR_SITE_DATA,
    HTTP_CONNECTION,
    HTTP_CONTENT_DIGEST,
    HTTP_CONTENT_DISPOSITION,
    HTTP_CONTENT_ENCODING,
    HTTP_CONTENT_LANGUAGE,
    HTTP_CONTENT_LENGTH,
    HTTP_CONTENT_LOCATION,
    HTTP_CONTENT_RANGE,
    HTTP_CONTENT_SECURITY_POLICY,
    HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY,
    HTTP_CONTENT_TYPE,
    HTTP_COOKIE,
    HTTP_CROSS_ORIGIN_EMBEDDER_POLICY,
    HTTP_CROSS_ORIGIN_OPENER_POLICY,
    HTTP_CROSS_ORIGIN_RESOURCE_POLICY,
    HTTP_DATE,
    HTTP_DEVICE_MEMORY,
    HTTP_ENTITY_TAG,
    HTTP_EXPECT,
    HTTP_EXPIRES,
    HTTP_FORWARDED,
    HTTP_FROM,
    HTTP_HOST,
    HTTP_IF_MATCH,
    HTTP_IF_MODIFIED_SINCE,
    HTTP_IF_NONE_MATCH,
    HTTP_IF_RANGE,
    HTTP_IF_UNMODIFIED_SINCE,
    HTTP_KEEP_ALIVE,
    HTTP_LAST_MODIFIED,
    HTTP_LINK,
    HTTP_LOCATION,
    HTTP_MAXIMUM_FORWARDS,
    HTTP_ORIGIN,
    HTTP_PRIORITY,
    HTTP_PROXY_AUTHENTICATION,
    HTTP_PROXY_AUTHORIZATION,
    HTTP_RANGE,
    HTTP_REFERER,
    HTTP_REFERRER_POLICY,
    HTTP_REFRESH,
    HTTP_REPR_DIGEST,
    HTTP_RETRY_AFTER,
    HTTP_SECURE_FETCH_DESTINATION,
    HTTP_SECURE_FETCH_MODE,
    HTTP_SECURE_FETCH_SITE,
    HTTP_SECURE_FETCH_USER,
    HTTP_SECURE_PURPOSE,
    HTTP_SECURE_WEBSOCKET_ACCEPT,
    HTTP_SECURE_WEBSOCKET_EXTENTIONS,
    HTTP_SECURE_WEBSOCKET_KEY,
    HTTP_SECURE_WEBSOCKET_PROTOCOL,
    HTTP_SECURE_WEBSOCKET_VERSION,
    HTTP_SERVER,
    HTTP_SERVER_TIMING,
    HTTP_SERVICE_WORKER,
    HTTP_SERVICE_WORKER_ALLOWED,
    HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD,
    HTTP_SET_COOKIE,
    HTTP_SOURCEMAP,
    HTTP_STRICT_TRANSPORT_SECURITY,
    HTTP_TRANSFER_ENCODING_TYPE,
    HTTP_TIMING_ALLOW_ORIGIN,
    HTTP_TRAILER,
    HTTP_TRANSFER_ENCODING,
    HTTP_UPGRADE,
    HTTP_UPGRADE_INSECURE_REQUESTS,
    HTTP_USER_AGENT,
    HTTP_VARY,
    HTTP_VIA_PROXY,
    HTTP_WANT_CONTENT_DIGEST,
    HTTP_WANT_REPRESENTATION_DIGEST,
    HTTP_WWW_AUTHENTICATE,
    HTTP_X_CONTENT_TYPE_OPTIONS,
    HTTP_X_FRAME_OPTIONS,
    httpHeaders
}

// Hypertext Transfer Protocol Headers
/** @type {string} */
const HTTP_ACCEPT = String("Accept");
/** @type {string} */
const HTTP_ACCEPT_CLIENT_HINT = String("Accept-CH");
/** @type {string} */
const HTTP_ACCEPT_ENCODING = String("Accept-Encoding");
/** @type {string} */
const HTTP_ACCEPT_LANGUAGE = String("Accept-Language");
/** @type {string} */
const HTTP_ACCEPT_PATCH = String("Accept-Patch");
/** @type {string} */
const HTTP_ACCEPT_POST = String("Accept-Post");
/** @type {string} */
const HTTP_ACCEPT_RANGES = String("Accept-Ranges");
/** @type {string} */
const HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS = String("Access-Control-Allow-Credentials");
/** @type {string} */
const HTTP_ACCESS_CONTROL_ALLOW_HEADERS = String("Access-Control-Allow-Headers");
/** @type {string} */
const HTTP_ACCESS_CONTROL_ALLOW_METHODS = String("Access-Control-Allow-Methods");
/** @type {string} */
const HTTP_ACCESS_CONTROL_ALLOW_ORIGIN = String("Access-Control-Allow-Origin");
/** @type {string} */
const HTTP_ACCESS_CONTROL_EXPOSE_HEADERS = String("Access-Control-Expose-Headers");
/** @type {string} */
const HTTP_ACCESS_CONTROL_MAX_AGE = String("Access-Control-Max-Age");
/** @type {string} */
const HTTP_ACCESS_CONTROL_REQUEST_HEADERS = String("Access-Control-Request-Headers");
/** @type {string} */
const HTTP_ACCESS_CONTROL_REQUEST_METHOD = String("Access-Control-Request-Method");
/** @type {string} */
const HTTP_AGE = String("Age");
/** @type {string} */
const HTTP_ALLOW = String("Allow");
/** @type {string} */
const HTTP_ALTERNATIVE_SERVICE = String("Alt-Svc");
/** @type {string} */
const HTTP_ALTERNATIVE_USED = String("Alt-Used");
/** @type {string} */
const HTTP_AUTHORIZATION = String("Authorization");
/** @type {string} */
const HTTP_CACHE_CONTROL = String("Cache-Control");
/** @type {string} */
const HTTP_CLEAR_SITE_DATA = String("Clear-Site-Data");
/** @type {string} */
const HTTP_CONNECTION = String("Connection");
/** @type {string} */
const HTTP_CONTENT_DIGEST = String("Content-Digest");
/** @type {string} */
const HTTP_CONTENT_DISPOSITION = String("Content-Disposition");
/** @type {string} */
const HTTP_CONTENT_ENCODING = String("Content-Encoding");
/** @type {string} */
const HTTP_CONTENT_LANGUAGE = String("Content-Language");
/** @type {string} */
const HTTP_CONTENT_LENGTH = String("Content-Length");
/** @type {string} */
const HTTP_CONTENT_LOCATION = String("Content-Location");
/** @type {string} */
const HTTP_CONTENT_RANGE = String("Content-Range");
/** @type {string} */
const HTTP_CONTENT_SECURITY_POLICY = String("Content-Security-Policy");
/** @type {string} */
const HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY = String("Content-Security-Policy-Report-Only");
/** @type {string} */
const HTTP_CONTENT_TYPE = String("Content-Type");
/** @type {string} */
const HTTP_COOKIE = String("Cookie");
/** @type {string} */
const HTTP_CROSS_ORIGIN_EMBEDDER_POLICY = String("Cross-Origin-Embedder-Policy");
/** @type {string} */
const HTTP_CROSS_ORIGIN_OPENER_POLICY = String("Cross-Origin-Opener-Policy");
/** @type {string} */
const HTTP_CROSS_ORIGIN_RESOURCE_POLICY = String("Cross-Origin-Resource-Policy");
/** @type {string} */
const HTTP_DATE = String("Date");
/** @type {string} */
const HTTP_DEVICE_MEMORY = String("Device-Memory");
/** @type {string} */
const HTTP_ENTITY_TAG = String("ETag");
/** @type {string} */
const HTTP_EXPECT = String("Expect");
/** @type {string} */
const HTTP_EXPIRES = String("Expires");
/** @type {string} */
const HTTP_FORWARDED = String("Forwarded");
/** @type {string} */
const HTTP_FROM = String("From");
/** @type {string} */
const HTTP_HOST = String("Host");
/** @type {string} */
const HTTP_IF_MATCH = String("If-Match");
/** @type {string} */
const HTTP_IF_MODIFIED_SINCE = String("If-Modified-Since");
/** @type {string} */
const HTTP_IF_NONE_MATCH = String("If-None-Match");
/** @type {string} */
const HTTP_IF_RANGE = String("If-Range");
/** @type {string} */
const HTTP_IF_UNMODIFIED_SINCE = String("If-Unmodified-Since");
/** @type {string} */
const HTTP_KEEP_ALIVE = String("Keep-Alive");
/** @type {string} */
const HTTP_LAST_MODIFIED = String("Last-Modified");
/** @type {string} */
const HTTP_LINK = String("Link");
/** @type {string} */
const HTTP_LOCATION = String("Location");
/** @type {string} */
const HTTP_MAXIMUM_FORWARDS = String("Max-Forwards");
/** @type {string} */
const HTTP_ORIGIN = String("Origin");
/** @type {string} */
const HTTP_PRIORITY = String("Priority");
/** @type {string} */
const HTTP_PROXY_AUTHENTICATION = String("Proxy-Authenticate");
/** @type {string} */
const HTTP_PROXY_AUTHORIZATION = String("Proxy-Authorization");
/** @type {string} */
const HTTP_RANGE = String("Range");
/** @type {string} */
const HTTP_REFERER = String("Referer");
/** @type {string} */
const HTTP_REFERRER_POLICY = String("Referrer-Policy");
/** @type {string} */
const HTTP_REFRESH = String("Refresh");
/** @type {string} */
const HTTP_REPR_DIGEST = String("Repr-Digest");
/** @type {string} */
const HTTP_RETRY_AFTER = String("Retry-After");
/** @type {string} */
const HTTP_SECURE_FETCH_DESTINATION = String("Sec-Fetch-Dest");
/** @type {string} */
const HTTP_SECURE_FETCH_MODE = String("Sec-Fetch-Mode");
/** @type {string} */
const HTTP_SECURE_FETCH_SITE = String("Sec-Fetch-Site");
/** @type {string} */
const HTTP_SECURE_FETCH_USER = String("Sec-Fetch-User");
/** @type {string} */
const HTTP_SECURE_PURPOSE = String("Sec-Purpose");
/** @type {string} */
const HTTP_SECURE_WEBSOCKET_ACCEPT = String("Sec-WebSocket-Accept");
/** @type {string} */
const HTTP_SECURE_WEBSOCKET_EXTENTIONS = String("Sec-WebSocket-Extensions");
/** @type {string} */
const HTTP_SECURE_WEBSOCKET_KEY = String("Sec-WebSocket-Key");
/** @type {string} */
const HTTP_SECURE_WEBSOCKET_PROTOCOL = String("Sec-WebSocket-Protocol");
/** @type {string} */
const HTTP_SECURE_WEBSOCKET_VERSION = String("Sec-WebSocket-Version");
/** @type {string} */
const HTTP_SERVER = String("Server");
/** @type {string} */
const HTTP_SERVER_TIMING = String("Server-Timing");
/** @type {string} */
const HTTP_SERVICE_WORKER = String("Service-Worker");
/** @type {string} */
const HTTP_SERVICE_WORKER_ALLOWED = String("Service-Worker-Allowed");
/** @type {string} */
const HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD = String("Service-Worker-Navigation-Preload");
/** @type {string} */
const HTTP_SET_COOKIE = String("Set-Cookie");
/** @type {string} */
const HTTP_SOURCEMAP = String("SourceMap");
/** @type {string} */
const HTTP_STRICT_TRANSPORT_SECURITY = String("Strict-Transport-Security");
/** @type {string} */
const HTTP_TRANSFER_ENCODING_TYPE = String("TE");
/** @type {string} */
const HTTP_TIMING_ALLOW_ORIGIN = String("Timing-Allow-Origin");
/** @type {string} */
const HTTP_TRAILER = String("Trailer");
/** @type {string} */
const HTTP_TRANSFER_ENCODING = String("Transfer-Encoding");
/** @type {string} */
const HTTP_UPGRADE = String("Upgrade");
/** @type {string} */
const HTTP_UPGRADE_INSECURE_REQUESTS = String("Upgrade-Insecure-Requests");
/** @type {string} */
const HTTP_USER_AGENT = String("User-Agent");
/** @type {string} */
const HTTP_VARY = String("Vary");
/** @type {string} */
const HTTP_VIA_PROXY = String("Via");
/** @type {string} */
const HTTP_WANT_CONTENT_DIGEST = String("Want-Content-Digest");
/** @type {string} */
const HTTP_WANT_REPRESENTATION_DIGEST = String("Want-Repr-Digest");
/** @type {string} */
const HTTP_WWW_AUTHENTICATE = String("WWW-Authenticate");
/** @type {string} */
const HTTP_X_CONTENT_TYPE_OPTIONS = String("X-Content-Type-Options");
/** @type {string} */
const HTTP_X_FRAME_OPTIONS = String("X-Frame-Options");

// Hypertext Transfer Protocol Header Vector
/** @type {function(): string[][]} */
function httpHeaders() {
    const hypertextTransferHeaders = new Array([
        HTTP_ACCEPT,
        HTTP_ACCEPT_CLIENT_HINT,
        HTTP_ACCEPT_ENCODING,
        HTTP_ACCEPT_LANGUAGE,
        HTTP_ACCEPT_PATCH,
        HTTP_ACCEPT_POST,
        HTTP_ACCEPT_RANGES,
        HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HTTP_ACCESS_CONTROL_ALLOW_HEADERS,
        HTTP_ACCESS_CONTROL_ALLOW_METHODS,
        HTTP_ACCESS_CONTROL_ALLOW_ORIGIN,
        HTTP_ACCESS_CONTROL_EXPOSE_HEADERS,
        HTTP_ACCESS_CONTROL_MAX_AGE,
        HTTP_ACCESS_CONTROL_REQUEST_HEADERS,
        HTTP_ACCESS_CONTROL_REQUEST_METHOD,
        HTTP_AGE,
        HTTP_ALLOW,
        HTTP_ALTERNATIVE_SERVICE,
        HTTP_ALTERNATIVE_USED,
        HTTP_AUTHORIZATION,
        HTTP_CACHE_CONTROL,
        HTTP_CLEAR_SITE_DATA,
        HTTP_CONNECTION,
        HTTP_CONTENT_DIGEST,
        HTTP_CONTENT_DISPOSITION,
        HTTP_CONTENT_ENCODING,
        HTTP_CONTENT_LANGUAGE,
        HTTP_CONTENT_LENGTH,
        HTTP_CONTENT_LOCATION,
        HTTP_CONTENT_RANGE,
        HTTP_CONTENT_SECURITY_POLICY,
        HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY,
        HTTP_CONTENT_TYPE,
        HTTP_COOKIE,
        HTTP_CROSS_ORIGIN_EMBEDDER_POLICY,
        HTTP_CROSS_ORIGIN_OPENER_POLICY,
        HTTP_CROSS_ORIGIN_RESOURCE_POLICY,
        HTTP_DATE,
        HTTP_DEVICE_MEMORY,
        HTTP_ENTITY_TAG,
        HTTP_EXPECT,
        HTTP_EXPIRES,
        HTTP_FORWARDED,
        HTTP_FROM,
        HTTP_HOST,
        HTTP_IF_MATCH,
        HTTP_IF_MODIFIED_SINCE,
        HTTP_IF_NONE_MATCH,
        HTTP_IF_RANGE,
        HTTP_IF_UNMODIFIED_SINCE,
        HTTP_KEEP_ALIVE,
        HTTP_LAST_MODIFIED,
        HTTP_LINK,
        HTTP_LOCATION,
        HTTP_MAXIMUM_FORWARDS,
        HTTP_ORIGIN,
        HTTP_PRIORITY,
        HTTP_PROXY_AUTHENTICATION,
        HTTP_PROXY_AUTHORIZATION,
        HTTP_RANGE,
        HTTP_REFERER,
        HTTP_REFERRER_POLICY,
        HTTP_REFRESH,
        HTTP_REPR_DIGEST,
        HTTP_RETRY_AFTER,
        HTTP_SECURE_FETCH_DESTINATION,
        HTTP_SECURE_FETCH_MODE,
        HTTP_SECURE_FETCH_SITE,
        HTTP_SECURE_FETCH_USER,
        HTTP_SECURE_PURPOSE,
        HTTP_SECURE_WEBSOCKET_ACCEPT,
        HTTP_SECURE_WEBSOCKET_EXTENTIONS,
        HTTP_SECURE_WEBSOCKET_KEY,
        HTTP_SECURE_WEBSOCKET_PROTOCOL,
        HTTP_SECURE_WEBSOCKET_VERSION,
        HTTP_SERVER,
        HTTP_SERVER_TIMING,
        HTTP_SERVICE_WORKER,
        HTTP_SERVICE_WORKER_ALLOWED,
        HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD,
        HTTP_SET_COOKIE,
        HTTP_SOURCEMAP,
        HTTP_STRICT_TRANSPORT_SECURITY,
        HTTP_TRANSFER_ENCODING_TYPE,
        HTTP_TIMING_ALLOW_ORIGIN,
        HTTP_TRAILER,
        HTTP_TRANSFER_ENCODING,
        HTTP_UPGRADE,
        HTTP_UPGRADE_INSECURE_REQUESTS,
        HTTP_USER_AGENT,
        HTTP_VARY,
        HTTP_VIA_PROXY,
        HTTP_WANT_CONTENT_DIGEST,
        HTTP_WANT_REPRESENTATION_DIGEST,
        HTTP_WWW_AUTHENTICATE,
        HTTP_X_CONTENT_TYPE_OPTIONS,
        HTTP_X_FRAME_OPTIONS
    ]);

    return hypertextTransferHeaders;
}
