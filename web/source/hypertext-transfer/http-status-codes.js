"use strict"

// @ts-check

export {
    HTTP_ONE_HUNDRED,
    HTTP_ONE_HUNDRED_ONE,
    HTTP_ONE_HUNDRED_TWO,
    HTTP_ONE_HUNDRED_THREE,
    HTTP_TWO_HUNDRED,
    HTTP_TWO_HUNDRED_ONE,
    HTTP_TWO_HUNDRED_TWO,
    HTTP_TWO_HUNDRED_THREE,
    HTTP_TWO_HUNDRED_FOUR,
    HTTP_TWO_HUNDRED_FIVE,
    HTTP_TWO_HUNDRED_SIX,
    HTTP_TWO_HUNDRED_SEVEN,
    HTTP_TWO_HUNDRED_EIGHT,
    HTTP_TWO_HUNDRED_TWENTY_SIX,
    HTTP_THREE_HUNDRED,
    HTTP_THREE_HUNDRED_ONE,
    HTTP_THREE_HUNDRED_TWO,
    HTTP_THREE_HUNDRED_THREE,
    HTTP_THREE_HUNDRED_FOUR,
    HTTP_THREE_HUNDRED_SEVEN,
    HTTP_THREE_HUNDRED_EIGHT,
    HTTP_FOUR_HUNDRED,
    HTTP_FOUR_HUNDRED_ONE,
    HTTP_FOUR_HUNDRED_TWO,
    HTTP_FOUR_HUNDRED_THREE,
    HTTP_FOUR_HUNDRED_FOUR,
    HTTP_FOUR_HUNDRED_FIVE,
    HTTP_FOUR_HUNDRED_SIX,
    HTTP_FOUR_HUNDRED_SEVEN,
    HTTP_FOUR_HUNDRED_EIGHT,
    HTTP_FOUR_HUNDRED_NINE,
    HTTP_FOUR_HUNDRED_TEN,
    HTTP_FOUR_HUNDRED_ELEVEN,
    HTTP_FOUR_HUNDRED_TWELVE,
    HTTP_FOUR_HUNDRED_THIRTEEN,
    HTTP_FOUR_HUNDRED_FOURTEEN,
    HTTP_FOUR_HUNDRED_FIFTEEN,
    HTTP_FOUR_HUNDRED_SIXTEEN,
    HTTP_FOUR_HUNDRED_SEVENTEEN,
    HTTP_FOUR_HUNDRED_EIGHTEEN,
    HTTP_FOUR_HUNDRED_TWENTY_ONE,
    HTTP_FOUR_HUNDRED_TWENTY_TWO,
    HTTP_FOUR_HUNDRED_TWENTY_THREE,
    HTTP_FOUR_HUNDRED_TWENTY_FOUR,
    HTTP_FOUR_HUNDRED_TWENTY_FIVE,
    HTTP_FOUR_HUNDRED_TWENTY_SIX,
    HTTP_FOUR_HUNDRED_TWENTY_EIGHT,
    HTTP_FOUR_HUNDRED_TWENTY_NINE,
    HTTP_FOUR_HUNDRED_THIRTY_ONE,
    HTTP_FOUR_HUNDRED_FIFTEY_ONE,
    HTTP_FIVE_HUNDRED,
    HTTP_FIVE_HUNDRED_ONE,
    HTTP_FIVE_HUNDRED_TWO,
    HTTP_FIVE_HUNDRED_THREE,
    HTTP_FIVE_HUNDRED_FOUR,
    HTTP_FIVE_HUNDRED_FIVE,
    HTTP_FIVE_HUNDRED_SIX,
    HTTP_FIVE_HUNDRED_SEVEN,
    HTTP_FIVE_HUNDRED_EIGHT,
    HTTP_FIVE_HUNDRED_TEN,
    HTTP_FIVE_HUNDRED_ELEVEN,
    HTTP_CONTINUE,
    HTTP_SWITCHING_PROTOCOLS,
    HTTP_PROCESSING,
    HTTP_EARLY_HINTS,
    HTTP_OK,
    HTTP_CREATED,
    HTTP_ACCEPTED,
    HTTP_NON_AUTHORITATIVE_INFORMATION,
    HTTP_NO_CONTENT,
    HTTP_RESET_CONTENT,
    HTTP_PARTIAL_CONTENT,
    HTTP_MULTI_STATUS,
    HTTP_ALREADY_REPORTED,
    HTTP_IM_USED,
    HTTP_MULTIPLE_CHOICES,
    HTTP_MOVED_PERMANENTLY,
    HTTP_FOUND,
    HTTP_SEE_OTHER,
    HTTP_NOT_MODIFIED,
    HTTP_TEMPORARY_REDIRECT,
    HTTP_PREMANENT_REDIRECT,
    HTTP_BAD_REQUEST,
    HTTP_UNAUTHORIZED,
    HTTP_PAYMENT_REQUIRED,
    HTTP_FORBIDDEN,
    HTTP_NOT_FOUND,
    HTTP_METHOD_NOT_ALLOWED,
    HTTP_NOT_ACCEPTABLE,
    HTTP_PROXY_AUTHENTICATION_REQUIRED,
    HTTP_REQUEST_TIMEOUT,
    HTTP_CONFLICT,
    HTTP_GONE,
    HTTP_LENGTH_REQUIRED,
    HTTP_PRECONDITION_FAILED,
    HTTP_CONTENT_TOO_LARGE,
    HTTP_URI_TOO_LONG,
    HTTP_UNSUPPORTED_MEDIA_TYPE,
    HTTP_RANGE_NOT_SATISFIABLE,
    HTTP_EXPECTATION_FAILED,
    HTTP_TEAPOT,
    HTTP_MISDIRECTED_REQUEST,
    HTTP_UNPROCESSABLE_CONTENT,
    HTTP_LOCKED,
    HTTP_FAILED_DEPENDENCY,
    HTTP_TOO_EARLY,
    HTTP_UPGRADE_REQUIRED,
    HTTP_PRECONDITION_REQUIRED,
    HTTP_TOO_MANY_REQUESTS,
    HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
    HTTP_UNAVAILABLE_FOR_LEGAL_REASONS,
    HTTP_INTERNAL_SERVER_ERROR,
    HTTP_NOT_IMPLEMENTED,
    HTTP_BAD_GATEWAY,
    HTTP_SERVICE_UNAVAILABLE,
    HTTP_GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    HTTP_VARIANT_ALSO_NEGOTIATES,
    HTTP_INSUFFICENT_STORAGE,
    HTTP_LOOP_DETECTED,
    HTTP_NOT_EXTENDED,
    HTTP_NETWORK_AUTHENTICATION_REQUIRED,
    httpStatusCodes
}

// Hypertext Transfer Protocol Status Codes
/** @type {string} */
const HTTP_ONE_HUNDRED = String("100");
/** @type {string} */
const HTTP_ONE_HUNDRED_ONE = String("101");
/** @type {string} */
const HTTP_ONE_HUNDRED_TWO = String("102");
/** @type {string} */
const HTTP_ONE_HUNDRED_THREE = String("103");
/** @type {string} */
const HTTP_TWO_HUNDRED = String("200");
/** @type {string} */
const HTTP_TWO_HUNDRED_ONE = String("201");
/** @type {string} */
const HTTP_TWO_HUNDRED_TWO = String("202");
/** @type {string} */
const HTTP_TWO_HUNDRED_THREE = String("203");
/** @type {string} */
const HTTP_TWO_HUNDRED_FOUR = String("204");
/** @type {string} */
const HTTP_TWO_HUNDRED_FIVE = String("205");
/** @type {string} */
const HTTP_TWO_HUNDRED_SIX = String("206");
/** @type {string} */
const HTTP_TWO_HUNDRED_SEVEN = String("207");
/** @type {string} */
const HTTP_TWO_HUNDRED_EIGHT = String("208");
/** @type {string} */
const HTTP_TWO_HUNDRED_TWENTY_SIX = String("226");
/** @type {string} */
const HTTP_THREE_HUNDRED = String("300");
/** @type {string} */
const HTTP_THREE_HUNDRED_ONE = String("301");
/** @type {string} */
const HTTP_THREE_HUNDRED_TWO = String("302");
/** @type {string} */
const HTTP_THREE_HUNDRED_THREE = String("303");
/** @type {string} */
const HTTP_THREE_HUNDRED_FOUR = String("304");
/** @type {string} */
const HTTP_THREE_HUNDRED_SEVEN = String("307");
/** @type {string} */
const HTTP_THREE_HUNDRED_EIGHT = String("308");
/** @type {string} */
const HTTP_FOUR_HUNDRED = String("400");
/** @type {string} */
const HTTP_FOUR_HUNDRED_ONE = String("401");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWO = String("402");
/** @type {string} */
const HTTP_FOUR_HUNDRED_THREE = String("403");
/** @type {string} */
const HTTP_FOUR_HUNDRED_FOUR = String("404");
/** @type {string} */
const HTTP_FOUR_HUNDRED_FIVE = String("405");
/** @type {string} */
const HTTP_FOUR_HUNDRED_SIX = String("406");
/** @type {string} */
const HTTP_FOUR_HUNDRED_SEVEN = String("407");
/** @type {string} */
const HTTP_FOUR_HUNDRED_EIGHT = String("408");
/** @type {string} */
const HTTP_FOUR_HUNDRED_NINE = String("409");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TEN = String("410");
/** @type {string} */
const HTTP_FOUR_HUNDRED_ELEVEN = String("411");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWELVE = String("412");
/** @type {string} */
const HTTP_FOUR_HUNDRED_THIRTEEN = String("413");
/** @type {string} */
const HTTP_FOUR_HUNDRED_FOURTEEN = String("414");
/** @type {string} */
const HTTP_FOUR_HUNDRED_FIFTEEN = String("415");
/** @type {string} */
const HTTP_FOUR_HUNDRED_SIXTEEN = String("416");
/** @type {string} */
const HTTP_FOUR_HUNDRED_SEVENTEEN = String("417");
/** @type {string} */
const HTTP_FOUR_HUNDRED_EIGHTEEN = String("418");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_ONE = String("421");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_TWO = String("422");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_THREE = String("423");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_FOUR = String("424");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_FIVE = String("425");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_SIX = String("426");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_EIGHT = String("428");
/** @type {string} */
const HTTP_FOUR_HUNDRED_TWENTY_NINE = String("429");
/** @type {string} */
const HTTP_FOUR_HUNDRED_THIRTY_ONE = String("431");
/** @type {string} */
const HTTP_FOUR_HUNDRED_FIFTEY_ONE = String("451");
/** @type {string} */
const HTTP_FIVE_HUNDRED = String("500");
/** @type {string} */
const HTTP_FIVE_HUNDRED_ONE = String("501");
/** @type {string} */
const HTTP_FIVE_HUNDRED_TWO = String("502");
/** @type {string} */
const HTTP_FIVE_HUNDRED_THREE = String("503");
/** @type {string} */
const HTTP_FIVE_HUNDRED_FOUR = String("504");
/** @type {string} */
const HTTP_FIVE_HUNDRED_FIVE = String("505");
/** @type {string} */
const HTTP_FIVE_HUNDRED_SIX = String("506");
/** @type {string} */
const HTTP_FIVE_HUNDRED_SEVEN = String("507");
/** @type {string} */
const HTTP_FIVE_HUNDRED_EIGHT = String("508");
/** @type {string} */
const HTTP_FIVE_HUNDRED_TEN = String("510");
/** @type {string} */
const HTTP_FIVE_HUNDRED_ELEVEN = String("511");

// Hypertext Transfer Protocol Status Texts
/** @type {string} */
const HTTP_CONTINUE = String("Continue");
/** @type {string} */
const HTTP_SWITCHING_PROTOCOLS = String("Switching Protocols");
/** @type {string} */
const HTTP_PROCESSING = String("Processing");
/** @type {string} */
const HTTP_EARLY_HINTS = String("Early Hints");
/** @type {string} */
const HTTP_OK = String("OK");
/** @type {string} */
const HTTP_CREATED = String("Created");
/** @type {string} */
const HTTP_ACCEPTED = String("Accepted");
/** @type {string} */
const HTTP_NON_AUTHORITATIVE_INFORMATION = String("Non-Authoritative Information");
/** @type {string} */
const HTTP_NO_CONTENT = String("No Content");
/** @type {string} */
const HTTP_RESET_CONTENT = String("Reset Content");
/** @type {string} */
const HTTP_PARTIAL_CONTENT = String("Partial Content");
/** @type {string} */
const HTTP_MULTI_STATUS = String("Multi-Status");
/** @type {string} */
const HTTP_ALREADY_REPORTED = String("Already Reported");
/** @type {string} */
const HTTP_IM_USED = String("IM Used");
/** @type {string} */
const HTTP_MULTIPLE_CHOICES = String("Multiple Choices");
/** @type {string} */
const HTTP_MOVED_PERMANENTLY = String("Moved Permanently");
/** @type {string} */
const HTTP_FOUND = String("Found");
/** @type {string} */
const HTTP_SEE_OTHER = String("See Other");
/** @type {string} */
const HTTP_NOT_MODIFIED = String("Not Modified");
/** @type {string} */
const HTTP_TEMPORARY_REDIRECT = String("Temporary Redirect");
/** @type {string} */
const HTTP_PREMANENT_REDIRECT = String("Permanent Redirect");
/** @type {string} */
const HTTP_BAD_REQUEST = String("Bad Request");
/** @type {string} */
const HTTP_UNAUTHORIZED = String("Unauthorized");
/** @type {string} */
const HTTP_PAYMENT_REQUIRED = String("Payment Required");
/** @type {string} */
const HTTP_FORBIDDEN = String("Forbidden");
/** @type {string} */
const HTTP_NOT_FOUND = String("Not Found");
/** @type {string} */
const HTTP_METHOD_NOT_ALLOWED = String("Method Not Allowed");
/** @type {string} */
const HTTP_NOT_ACCEPTABLE = String("Not Acceptable");
/** @type {string} */
const HTTP_PROXY_AUTHENTICATION_REQUIRED = String("Proxy Authentication Required");
/** @type {string} */
const HTTP_REQUEST_TIMEOUT = String("Request Timeout");
/** @type {string} */
const HTTP_CONFLICT = String("Conflict");
/** @type {string} */
const HTTP_GONE = String("Gone");
/** @type {string} */
const HTTP_LENGTH_REQUIRED = String("Length Required");
/** @type {string} */
const HTTP_PRECONDITION_FAILED = String("Precondition Failed");
/** @type {string} */
const HTTP_CONTENT_TOO_LARGE = String("Content Too Large");
/** @type {string} */
const HTTP_URI_TOO_LONG = String("URI Too Long");
/** @type {string} */
const HTTP_UNSUPPORTED_MEDIA_TYPE = String("Unsupported Media Type");
/** @type {string} */
const HTTP_RANGE_NOT_SATISFIABLE = String("Range Not Satisfiable");
/** @type {string} */
const HTTP_EXPECTATION_FAILED = String("Expectation Failed");
/** @type {string} */
const HTTP_TEAPOT = String("I'm a teapot");
/** @type {string} */
const HTTP_MISDIRECTED_REQUEST = String("Misdirected Request");
/** @type {string} */
const HTTP_UNPROCESSABLE_CONTENT = String("Unprocessable Content");
/** @type {string} */
const HTTP_LOCKED = String("Locked");
/** @type {string} */
const HTTP_FAILED_DEPENDENCY = String("Failed Dependency");
/** @type {string} */
const HTTP_TOO_EARLY = String("Too Early");
/** @type {string} */
const HTTP_UPGRADE_REQUIRED = String("Upgrade Required");
/** @type {string} */
const HTTP_PRECONDITION_REQUIRED = String("Precondition Required");
/** @type {string} */
const HTTP_TOO_MANY_REQUESTS = String("Too Many Requests");
/** @type {string} */
const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE = String("Request Header Fields Too Large");
/** @type {string} */
const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS = String("Unavailable For Legal Reasons");
/** @type {string} */
const HTTP_INTERNAL_SERVER_ERROR = String("Internal Server Error");
/** @type {string} */
const HTTP_NOT_IMPLEMENTED = String("Not Implemented");
/** @type {string} */
const HTTP_BAD_GATEWAY = String("Bad Gateway");
/** @type {string} */
const HTTP_SERVICE_UNAVAILABLE = String("Service Unavailable");
/** @type {string} */
const HTTP_GATEWAY_TIMEOUT = String("Gateway Timeout");
/** @type {string} */
const HTTP_VERSION_NOT_SUPPORTED = String("HTTP Version Not Supported");
/** @type {string} */
const HTTP_VARIANT_ALSO_NEGOTIATES = String("Variant Also Negotiates");
/** @type {string} */
const HTTP_INSUFFICENT_STORAGE = String("Insufficient Storage");
/** @type {string} */
const HTTP_LOOP_DETECTED = String("Loop Detected");
/** @type {string} */
const HTTP_NOT_EXTENDED = String("Not Extended");
/** @type {string} */
const HTTP_NETWORK_AUTHENTICATION_REQUIRED = String("Network Authentication Required");

// Hypertext Transfer Protocol Status Code and Status Text Map
/** @type {function(): Map<string, string>} */
function httpStatusCodes() {
    const hypertextTransferStatusCodes = new Map([
        [HTTP_ONE_HUNDRED, HTTP_CONTINUE],
        [HTTP_ONE_HUNDRED_ONE, HTTP_SWITCHING_PROTOCOLS],
        [HTTP_ONE_HUNDRED_TWO, HTTP_PROCESSING],
        [HTTP_ONE_HUNDRED_THREE, HTTP_EARLY_HINTS],
        [HTTP_TWO_HUNDRED, HTTP_OK],
        [HTTP_TWO_HUNDRED_ONE, HTTP_CREATED],
        [HTTP_TWO_HUNDRED_TWO, HTTP_ACCEPTED],
        [HTTP_TWO_HUNDRED_THREE, HTTP_NON_AUTHORITATIVE_INFORMATION],
        [HTTP_TWO_HUNDRED_FOUR, HTTP_NO_CONTENT],
        [HTTP_TWO_HUNDRED_FIVE, HTTP_RESET_CONTENT],
        [HTTP_TWO_HUNDRED_SIX, HTTP_PARTIAL_CONTENT],
        [HTTP_TWO_HUNDRED_SEVEN, HTTP_MULTI_STATUS],
        [HTTP_TWO_HUNDRED_EIGHT, HTTP_ALREADY_REPORTED],
        [HTTP_TWO_HUNDRED_TWENTY_SIX, HTTP_IM_USED],
        [HTTP_THREE_HUNDRED, HTTP_MULTIPLE_CHOICES],
        [HTTP_THREE_HUNDRED_ONE, HTTP_MOVED_PERMANENTLY],
        [HTTP_THREE_HUNDRED_TWO, HTTP_FOUND],
        [HTTP_THREE_HUNDRED_THREE, HTTP_SEE_OTHER],
        [HTTP_THREE_HUNDRED_FOUR, HTTP_NOT_MODIFIED],
        [HTTP_THREE_HUNDRED_SEVEN, HTTP_TEMPORARY_REDIRECT],
        [HTTP_THREE_HUNDRED_EIGHT, HTTP_PREMANENT_REDIRECT],
        [HTTP_FOUR_HUNDRED, HTTP_BAD_REQUEST],
        [HTTP_FOUR_HUNDRED_ONE, HTTP_UNAUTHORIZED],
        [HTTP_FOUR_HUNDRED_TWO, HTTP_PAYMENT_REQUIRED],
        [HTTP_FOUR_HUNDRED_THREE, HTTP_FORBIDDEN],
        [HTTP_FOUR_HUNDRED_FOUR, HTTP_NOT_FOUND],
        [HTTP_FOUR_HUNDRED_FIVE, HTTP_METHOD_NOT_ALLOWED],
        [HTTP_FOUR_HUNDRED_SIX, HTTP_NOT_ACCEPTABLE],
        [HTTP_FOUR_HUNDRED_SEVEN, HTTP_PROXY_AUTHENTICATION_REQUIRED],
        [HTTP_FOUR_HUNDRED_EIGHT, HTTP_REQUEST_TIMEOUT],
        [HTTP_FOUR_HUNDRED_NINE, HTTP_CONFLICT],
        [HTTP_FOUR_HUNDRED_TEN, HTTP_GONE],
        [HTTP_FOUR_HUNDRED_ELEVEN, HTTP_LENGTH_REQUIRED],
        [HTTP_FOUR_HUNDRED_TWELVE, HTTP_PRECONDITION_FAILED],
        [HTTP_FOUR_HUNDRED_THIRTEEN, HTTP_CONTENT_TOO_LARGE],
        [HTTP_FOUR_HUNDRED_FOURTEEN, HTTP_URI_TOO_LONG],
        [HTTP_FOUR_HUNDRED_FIFTEEN, HTTP_UNSUPPORTED_MEDIA_TYPE],
        [HTTP_FOUR_HUNDRED_SIXTEEN, HTTP_RANGE_NOT_SATISFIABLE],
        [HTTP_FOUR_HUNDRED_SEVENTEEN, HTTP_EXPECTATION_FAILED],
        [HTTP_FOUR_HUNDRED_EIGHTEEN, HTTP_TEAPOT],
        [HTTP_FOUR_HUNDRED_TWENTY_ONE, HTTP_MISDIRECTED_REQUEST],
        [HTTP_FOUR_HUNDRED_TWENTY_TWO, HTTP_UNPROCESSABLE_CONTENT],
        [HTTP_FOUR_HUNDRED_TWENTY_THREE, HTTP_LOCKED],
        [HTTP_FOUR_HUNDRED_TWENTY_FOUR, HTTP_FAILED_DEPENDENCY],
        [HTTP_FOUR_HUNDRED_TWENTY_FIVE, HTTP_TOO_EARLY],
        [HTTP_FOUR_HUNDRED_TWENTY_SIX, HTTP_UPGRADE_REQUIRED],
        [HTTP_FOUR_HUNDRED_TWENTY_EIGHT, HTTP_PRECONDITION_REQUIRED],
        [HTTP_FOUR_HUNDRED_TWENTY_NINE, HTTP_TOO_MANY_REQUESTS],
        [HTTP_FOUR_HUNDRED_THIRTY_ONE, HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE],
        [HTTP_FOUR_HUNDRED_FIFTEY_ONE, HTTP_UNAVAILABLE_FOR_LEGAL_REASONS],
        [HTTP_FIVE_HUNDRED, HTTP_INTERNAL_SERVER_ERROR],
        [HTTP_FIVE_HUNDRED_ONE, HTTP_NOT_IMPLEMENTED],
        [HTTP_FIVE_HUNDRED_TWO, HTTP_BAD_GATEWAY],
        [HTTP_FIVE_HUNDRED_THREE, HTTP_SERVICE_UNAVAILABLE],
        [HTTP_FIVE_HUNDRED_FOUR, HTTP_GATEWAY_TIMEOUT],
        [HTTP_FIVE_HUNDRED_FIVE, HTTP_VERSION_NOT_SUPPORTED],
        [HTTP_FIVE_HUNDRED_SIX, HTTP_VARIANT_ALSO_NEGOTIATES],
        [HTTP_FIVE_HUNDRED_SEVEN, HTTP_INSUFFICENT_STORAGE],
        [HTTP_FIVE_HUNDRED_EIGHT, HTTP_LOOP_DETECTED],
        [HTTP_FIVE_HUNDRED_TEN, HTTP_NOT_EXTENDED],
        [HTTP_FIVE_HUNDRED_ELEVEN, HTTP_NETWORK_AUTHENTICATION_REQUIRED],
    ]);

    return hypertextTransferStatusCodes;
}
