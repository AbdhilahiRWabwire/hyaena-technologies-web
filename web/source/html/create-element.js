"use strict";

export {
    htmlAnchor,
    htmlAbbreviation,
    htmlContactAddress,
    htmlImageMapArea,
    htmlArticleContents,
    htmlAside,
    htmlEmbedAudio,
    htmlBringAttentionTo,
    htmlDocumentBaseURL,
    htmlBidirectionalIsolate,
    htmlBidirectionalTextOverride,
    htmlBlockQuotation,
    htmlDocumentBody,
    htmlLineBreak,
    htmlButton,
    htmlGraphicsCanvas,
    htmlTableCaption,
    htmlCitation,
    htmlInlineCode,
    htmlTableColumn,
    htmlTableColumnGroup,
    htmlData,
    htmlDataList,
    htmlDescriptionDetails,
    htmlDeletedText,
    htmlDetailsDisclosure,
    htmlDefinition,
    htmlDialog,
    htmlContentDivision,
    htmlDescriptionList,
    htmlDescriptionTerm,
    htmlEmphasis,
    htmlEmbedExternalContent,
    htmlFieldSet,
    htmlFigureCaption,
    htmlFigureWithOptionalCaption,
    htmlFooter,
    htmlForm,
    htmlSectionHeading,
    htmlDocumentMetadataHeader,
    htmlHeader,
    htmlHeadergGroup,
    htmlThematicBreakHorizontalRule,
    htmlDocumentRoot,
    htmlIdiomaticText,
    htmlInlineFrame,
    htmlImageEmbed,
    htmlInput,
    htmlInsertedText,
    htmlKeyboardInput,
    htmlLabel,
    htmlFieldSetLegend,
    htmlListItem,
    htmlExternalResourceLink,
    htmlMain,
    htmlImageMap,
    htmlMarkText,
    htmlMenu,
    htmlMetadata,
    htmlMeter,
    htmlNavigationSection,
    htmlNoScript,
    htmlExternalObject,
    htmlOrderedList,
    htmlOptionGroup,
    htmlOption,
    htmlOutput,
    htmlParagraph,
    htmlPicture,
    htmlPreformattedText,
    htmlProgressIndicator,
    htmlInlineQuotation,
    htmlRubyFallbackParenthesis,
    htmlRubyText,
    htmlRubyAnnotation,
    htmlStrikeThrough,
    htmlSampleOutput,
    htmlScript,
    htmlGenericSearch,
    htmlGenericSection,
    htmlSelect,
    htmlWebComponentSlot,
    htmlSideComment,
    htmlMediaOrImageSource,
    htmlContentSpan,
    htmlStrongImportance,
    htmlStyleInformation,
    htmlSubScript,
    htmlDisclosureSummary,
    htmlSuperScript,
    htmlTable,
    htmlTableBody,
    htmlTableDataCell,
    htmlContentTemplate,
    htmlTextArea,
    htmlTableFoot,
    htmlTableHeader,
    htmlTableHead,
    htmlDateTime,
    htmlDocumentTitle,
    htmlTableRow,
    htmlEmbedTextTrack,
    htmlUnarticulatedAnnotationUnderline,
    htmlUnorderedList,
    htmlVariable,
    htmlVideoEmbed,
    htmlLineBreakOpportunity
};

// @ts-check

// Hypertext Markup Language Elements

// Anchor Element
/** @type {HTMLAnchorElement} */
const htmlAnchor = document.createElement("a");

// Abbreviation Element
/** @type {HTMLElement} */
const htmlAbbreviation = document.createElement("abbr");

// Contact Address Element
/** @type {HTMLElement} */
const htmlContactAddress = document.createElement("address");

// Image Map Area Element
/** @type {HTMLAreaElement} */
const htmlImageMapArea = document.createElement("area");

// Article Contents Element
/** @type {HTMLElement} */
const htmlArticleContents = document.createElement("article");

// Aside Element
/** @type {HTMLElement} */
const htmlAside = document.createElement("aside");

// Embed Audio Element
/** @type {HTMLAudioElement} */
const htmlEmbedAudio = document.createElement("audio");

// Bring Attention To Element
/** @type {HTMLElement} */
const htmlBringAttentionTo = document.createElement("b");

// Document Base URL Element
/** @type {HTMLBaseElement} */
const htmlDocumentBaseURL = document.createElement("base");

// Bidirectional Isolate Element
/** @type {HTMLElement} */
const htmlBidirectionalIsolate = document.createElement("bdi");

// Bidirectional Text Override Element
/** @type {HTMLElement} */
const htmlBidirectionalTextOverride = document.createElement("bdo");

// Block Quotation Element
/** @type {HTMLQuoteElement} */
const htmlBlockQuotation = document.createElement("blockquote");

// Document Body Element
/** @type {HTMLBodyElement} */
const htmlDocumentBody = document.createElement("body");

// Line Break Element
/** @type {HTMLBRElement} */
const htmlLineBreak = document.createElement("br");

// Button Element
/** @type {HTMLButtonElement} */
const htmlButton = document.createElement("button");

// Graphics Canvas Element
/** @type {HTMLCanvasElement} */
const htmlGraphicsCanvas = document.createElement("canvas");

// Table Caption Element
/** @type {HTMLTableCaptionElement} */
const htmlTableCaption = document.createElement("caption");

// Citation Element
/** @type {HTMLElement} */
const htmlCitation = document.createElement("cite");

// Inline Code Element
/** @type {HTMLElement} */
const htmlInlineCode = document.createElement("code");

// Table Column Element
/** @type {HTMLTableColElement} */
const htmlTableColumn = document.createElement("col");

// Table Column Group Element
/** @type {HTMLTableColElement} */
const htmlTableColumnGroup = document.createElement("colgroup");

// Data Element
/** @type {HTMLDataElement} */
const htmlData = document.createElement("data");

// Data List Element
/** @type {HTMLDataListElement} */
const htmlDataList = document.createElement("datalist");

// Description Details Element
/** @type {HTMLElement} */
const htmlDescriptionDetails = document.createElement("dd");

// Deleted Text Element
/** @type {HTMLModElement} */
const htmlDeletedText = document.createElement("del");

// Details Disclosure Element
/** @type {HTMLDetailsElement} */
const htmlDetailsDisclosure = document.createElement("details");

// Definition Element
/** @type {HTMLElement} */
const htmlDefinition = document.createElement("dfn");

// Dialog Element
/** @type {HTMLDialogElement} */
const htmlDialog = document.createElement("dialog");

// Content Division Element
/** @type {HTMLDivElement} */
const htmlContentDivision = document.createElement("div");

// Description List Element
/** @type {HTMLDListElement} */
const htmlDescriptionList = document.createElement("dl");

// Description Term Element
/** @type {HTMLElement} */
const htmlDescriptionTerm = document.createElement("dt");

// Emphasis Element
/** @type {HTMLElement} */
const htmlEmphasis = document.createElement("em");

// Embed External Content Element
/** @type {HTMLEmbedElement} */
const htmlEmbedExternalContent = document.createElement("embed");

// Field Set Element
/** @type {HTMLFieldSetElement} */
const htmlFieldSet = document.createElement("fieldset");

// Figure Caption Element
/** @type {HTMLElement} */
const htmlFigureCaption = document.createElement("figcaption");

// Figure with Optional Caption Element
/** @type {HTMLElement} */
const htmlFigureWithOptionalCaption = document.createElement("figure");

// Footer Element
/** @type {HTMLElement} */
const htmlFooter = document.createElement("footer");

// Form Element
/** @type {HTMLFormElement} */
const htmlForm = document.createElement("form");

// Section Heading Element
/** @type {HTMLHeadingElement} */
const htmlSectionHeading = document.createElement("h1");

// Document Metadata Header Element
/** @type {HTMLHeadElement} */
const htmlDocumentMetadataHeader = document.createElement("head");

// Header Element
/** @type {HTMLElement} */
const htmlHeader = document.createElement("header");

// Header Group Element
/** @type {HTMLElement} */
const htmlHeadergGroup = document.createElement("hgroup");

// Thematic Break Horizontal Rule Element
/** @type {HTMLHRElement} */
const htmlThematicBreakHorizontalRule = document.createElement("hr");

// HTML Document Root Element
/** @type {HTMLHtmlElement} */
const htmlDocumentRoot = document.createElement("html");

// Idiomatic Text Element
/** @type {HTMLElement} */
const htmlIdiomaticText = document.createElement("i");

// Inline Frame Element
/** @type {HTMLIFrameElement} */
const htmlInlineFrame = document.createElement("iframe");

// Image Embed Element
/** @type {HTMLImageElement} */
const htmlImageEmbed = document.createElement("img");

// Input Element
/** @type {HTMLInputElement} */
const htmlInput = document.createElement("input");

// Inserted Text Elemen
/** @type {HTMLModElement} */
const htmlInsertedText = document.createElement("ins");

// Keyboard Input Element
/** @type {HTMLElement} */
const htmlKeyboardInput = document.createElement("kbd");

// Label Element
/** @type {HTMLLabelElement} */
const htmlLabel = document.createElement("label");

// Field Set Legend Element
/** @type {HTMLLegendElement} */
const htmlFieldSetLegend = document.createElement("legend");

// List Item Element
/** @type {HTMLLIElement} */
const htmlListItem = document.createElement("li");

// External Resource Link Element
/** @type {HTMLLinkElement} */
const htmlExternalResourceLink = document.createElement("link");

// Main Element
/** @type {HTMLElement} */
const htmlMain = document.createElement("main");

// Image Map Element
/** @type {HTMLMapElement} */
const htmlImageMap = document.createElement("map");

// Mark Text Element
/** @type {HTMLElement} */
const htmlMarkText = document.createElement("mark");

// Menu Element
/** @type {HTMLMenuElement} */
const htmlMenu = document.createElement("menu");

// Metadata Element
/** @type {HTMLMetaElement} */
const htmlMetadata = document.createElement("meta");

// Meter Element
/** @type {HTMLMeterElement} */
const htmlMeter = document.createElement("meter");

// Navigation Section Element
/** @type {HTMLElement} */
const htmlNavigationSection = document.createElement("nav");

// Noscript Element
/** @type {HTMLElement} */
const htmlNoScript = document.createElement("noscript");

// External Object Element
/** @type {HTMLObjectElement} */
const htmlExternalObject = document.createElement("object");

// Ordered List Element
/** @type {HTMLOListElement} */
const htmlOrderedList = document.createElement("ol");

// Option Group Element
/** @type {HTMLOptGroupElement} */
const htmlOptionGroup = document.createElement("optgroup");

// Option Element
/** @type {HTMLOptionElement} */
const htmlOption = document.createElement("option");

// Output Element
/** @type {HTMLOutputElement} */
const htmlOutput = document.createElement("output");

// Paragraph Element
/** @type {HTMLParagraphElement} */
const htmlParagraph = document.createElement("p");

// Picture Element
/** @type {HTMLPictureElement} */
const htmlPicture = document.createElement("picture");

// Preformatted Text Element
/** @type {HTMLPreElement} */
const htmlPreformattedText = document.createElement("pre");

// Progress Indicator Element
/** @type {HTMLProgressElement} */
const htmlProgressIndicator = document.createElement("progress");

// Inline Quotation Element
/** @type {HTMLQuoteElement} */
const htmlInlineQuotation = document.createElement("q");

// Ruby Fallback Parenthesis Element
/** @type {HTMLElement} */
const htmlRubyFallbackParenthesis = document.createElement("rp");

// Ruby Text Element
/** @type {HTMLElement} */
const htmlRubyText = document.createElement("rt");

// Ruby Annotation Element
/** @type {HTMLElement} */
const htmlRubyAnnotation = document.createElement("ruby");

// Strike Through Element
/** @type {HTMLElement} */
const htmlStrikeThrough = document.createElement("s");

// Sample Output Element
/** @type {HTMLElement} */
const htmlSampleOutput = document.createElement("samp");

// Script Element
/** @type {HTMLScriptElement} */
const htmlScript = document.createElement("script");

// Generic Search Element
/** @type {HTMLElement} */
const htmlGenericSearch = document.createElement("search");

// Generic Search Element
/** @type {HTMLElement} */
const htmlGenericSection = document.createElement("section");

// Select Element
/** @type {HTMLSelectElement} */
const htmlSelect = document.createElement("select");

// Web Component Slot Element
/** @type {HTMLSlotElement} */
const htmlWebComponentSlot = document.createElement("slot");

// Side Comment Element
/** @type {HTMLElement} */
const htmlSideComment = document.createElement("small");

// Media or Image Source Element
/** @type {HTMLSourceElement} */
const htmlMediaOrImageSource = document.createElement("source");

// Content Span Element
/** @type {HTMLSpanElement} */
const htmlContentSpan = document.createElement("span");

// Strong Importance Element
/** @type {HTMLElement} */
const htmlStrongImportance = document.createElement("strong");

// Style Information Element
/** @type {HTMLStyleElement} */
const htmlStyleInformation = document.createElement("style");

// Subscript Element
/** @type {HTMLElement} */
const htmlSubScript = document.createElement("sub");

// Disclosure Summary Element
/** @type {HTMLElement} */
const htmlDisclosureSummary = document.createElement("summary");

// Superscript Element
/** @type {HTMLElement} */
const htmlSuperScript = document.createElement("sup");

// Table Element
/** @type {HTMLTableElement} */
const htmlTable = document.createElement("table");

// Table Body Element
/** @type {HTMLTableSectionElement} */
const htmlTableBody = document.createElement("tbody");

// Table Data Cell Element
/** @type {HTMLTableCellElement} */
const htmlTableDataCell = document.createElement("td");

// Content Template Element
/** @type {HTMLTemplateElement} */
const htmlContentTemplate = document.createElement("template");

// Text Area Element
/** @type {HTMLTextAreaElement} */
const htmlTextArea = document.createElement("textarea");

// Table Foot Element
/** @type {HTMLTableSectionElement} */
const htmlTableFoot = document.createElement("tfoot");

// Table Header Element
/** @type {HTMLTableCellElement} */
const htmlTableHeader = document.createElement("th");

// Table Head Element
/** @type {HTMLTableSectionElement} */
const htmlTableHead = document.createElement("thead");

// Date Time Element
/** @type {HTMLTimeElement} */
const htmlDateTime = document.createElement("time");

// Document Title Element
/** @type {HTMLTitleElement} */
const htmlDocumentTitle  = document.createElement("title");

// Table Row Element
/** @type {HTMLTableRowElement} */
const htmlTableRow = document.createElement("tr");

// Embed Text Track Element
/** @type {HTMLTrackElement} */
const htmlEmbedTextTrack = document.createElement("track");

// Unarticulated Annotation Underline Element
/** @type {HTMLElement} */
const htmlUnarticulatedAnnotationUnderline = document.createElement("u");

// Unordered List Element
/** @type {HTMLUListElement} */
const htmlUnorderedList = document.createElement("ul");

// Variable Element
/** @type {HTMLElement} */
const htmlVariable = document.createElement("var");

// Video Embed Element
/** @type {HTMLVideoElement} */
const htmlVideoEmbed = document.createElement("video");

// Line Break Opportunity Element
/** @type {HTMLElement} */
const htmlLineBreakOpportunity = document.createElement("wbr");
