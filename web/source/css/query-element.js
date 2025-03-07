"use strict";

export {
    queryHTMLAnchor,
    queryHTMLAbbreviation,
    queryHTMLContactAddress,
    queryHTMLImageMapArea,
    queryHTMLArticleContents,
    queryHTMLAside,
    queryHTMLEmbedAudio,
    queryHTMLBringAttentionTo,
    queryHTMLDocumentBaseURL,
    queryHTMLBidirectionalIsolate,
    queryHTMLBidirectionalTextOverride,
    queryHTMLBlockQuotation,
    queryHTMLDocumentBody,
    queryHTMLLineBreak,
    queryHTMLButton,
    queryHTMLGraphicsCanvas,
    queryHTMLTableCaption,
    queryHTMLCitation,
    queryHTMLInlineCode,
    queryHTMLTableColumn,
    queryHTMLTableColumnGroup,
    queryHTMLData,
    queryHTMLDataList,
    queryHTMLDescriptionDetails,
    queryHTMLDeletedText,
    queryHTMLDetailsDisclosure,
    queryHTMLDefinition,
    queryHTMLDialog,
    queryHTMLContentDivision,
    queryHTMLDescriptionList,
    queryHTMLDescriptionTerm,
    queryHTMLEmphasis,
    queryHTMLEmbedExternalContent,
    queryHTMLFieldSet,
    queryHTMLFigureCaption,
    queryHTMLFigureWithOptionalCaption,
    queryHTMLFooter,
    queryHTMLForm,
    queryHTMLSectionHeading,
    queryHTMLDocumentMetadataHeader,
    queryHTMLHeader,
    queryHTMLHeadergGroup,
    queryHTMLThematicBreakHorizontalRule,
    queryHTMLDocumentRoot,
    queryHTMLIdiomaticText,
    queryHTMLInlineFrame,
    queryHTMLImageEmbed,
    queryHTMLInput,
    queryHTMLInsertedText,
    queryHTMLKeyboardInput,
    queryHTMLLabel,
    queryHTMLFieldSetLegend,
    queryHTMLListItem,
    queryHTMLExternalResourceLink,
    queryHTMLMain,
    queryHTMLImageMap,
    queryHTMLMarkText,
    queryHTMLMenu,
    queryHTMLMetadata,
    queryHTMLMeter,
    queryHTMLNavigationSection,
    queryHTMLNoScript,
    queryHTMLExternalObject,
    queryHTMLOrderedList,
    queryHTMLOptionGroup,
    queryHTMLOption,
    queryHTMLOutput,
    queryHTMLParagraph,
    queryHTMLPicture,
    queryHTMLPreformattedText,
    queryHTMLProgressIndicator,
    queryHTMLInlineQuotation,
    queryHTMLRubyFallbackParenthesis,
    queryHTMLRubyText,
    queryHTMLRubyAnnotation,
    queryHTMLStrikeThrough,
    queryHTMLSampleOutput,
    queryHTMLScript,
    queryHTMLGenericSearch,
    queryHTMLGenericSection,
    queryHTMLSelect,
    queryHTMLWebComponentSlot,
    queryHTMLSideComment,
    queryHTMLMediaOrImageSource,
    queryHTMLContentSpan,
    queryHTMLStrongImportance,
    queryHTMLStyleInformation,
    queryHTMLSubScript,
    queryHTMLDisclosureSummary,
    queryHTMLSuperScript,
    queryHTMLTable,
    queryHTMLTableBody,
    queryHTMLTableDataCell,
    queryHTMLContentTemplate,
    queryHTMLTextArea,
    queryHTMLTableFoot,
    queryHTMLTableHeader,
    queryHTMLTableHead,
    queryHTMLDateTime,
    queryHTMLDocumentTitle,
    queryHTMLTableRow,
    queryHTMLEmbedTextTrack,
    queryHTMLUnarticulatedAnnotationUnderline,
    queryHTMLUnorderedList,
    queryHTMLVariable,
    queryHTMLVideoEmbed,
    queryHTMLLineBreakOpportunity
};

// @ts-check

// Cascading Style Sheet Selectors - Query Hypertext Markup Language Elements

// Query Anchor Element
/** @type {HTMLAnchorElement | null} */
const queryHTMLAnchor = document.querySelector("a");

// Query Abbreviation Element
/** @type {HTMLElement | null} */
const queryHTMLAbbreviation = document.querySelector("abbr");

// Query Contact Address Element
/** @type {HTMLElement | null} */
const queryHTMLContactAddress = document.querySelector("address");

// Query Image Map Area Element
/** @type {HTMLAreaElement | null} */
const queryHTMLImageMapArea = document.querySelector("area");

// Query Article Contents Element
/** @type {HTMLElement | null} */
const queryHTMLArticleContents = document.querySelector("article");

// Query Aside Element
/** @type {HTMLElement | null} */
const queryHTMLAside = document.querySelector("aside");

// Query Embed Audio Element
/** @type {HTMLAudioElement | null} */
const queryHTMLEmbedAudio = document.querySelector("audio");

// Query Bring Attention To Element
/** @type {HTMLElement | null} */
const queryHTMLBringAttentionTo = document.querySelector("b");

// Query Document Base URL Element
/** @type {HTMLBaseElement | null} */
const queryHTMLDocumentBaseURL = document.querySelector("base");

// Query Bidirectional Isolate Element
/** @type {HTMLElement | null} */
const queryHTMLBidirectionalIsolate = document.querySelector("bdi");

// Query Bidirectional Text Override Element
/** @type {HTMLElement | null} */
const queryHTMLBidirectionalTextOverride = document.querySelector("bdo");

// Query Block Quotation Element
/** @type {HTMLQuoteElement | null} */
const queryHTMLBlockQuotation = document.querySelector("blockquote");

// Query Document Body Element
/** @type {HTMLBodyElement | null} */
const queryHTMLDocumentBody = document.querySelector("body");

// Query Line Break Element
/** @type {HTMLBRElement | null} */
const queryHTMLLineBreak = document.querySelector("br");

// Query Button Element
/** @type {HTMLButtonElement | null} */
const queryHTMLButton = document.querySelector("button");

// Query Graphics Canvas Element
/** @type {HTMLCanvasElement | null} */
const queryHTMLGraphicsCanvas = document.querySelector("canvas");

// Query Table Caption Element
/** @type {HTMLTableCaptionElement | null} */
const queryHTMLTableCaption = document.querySelector("caption");

// Query Citation Element
/** @type {HTMLElement | null} */
const queryHTMLCitation = document.querySelector("cite");

// Query Inline Code Element
/** @type {HTMLElement | null} */
const queryHTMLInlineCode = document.querySelector("code");

// Query Table Column Element
/** @type {HTMLTableColElement | null} */
const queryHTMLTableColumn = document.querySelector("col");

// Query Table Column Group Element
/** @type {HTMLTableColElement | null} */
const queryHTMLTableColumnGroup = document.querySelector("colgroup");

// Query Data Element
/** @type {HTMLDataElement | null} */
const queryHTMLData = document.querySelector("data");

// Query Data List Element
/** @type {HTMLDataListElement | null} */
const queryHTMLDataList = document.querySelector("datalist");

// Query Description Details Element
/** @type {HTMLElement | null} */
const queryHTMLDescriptionDetails = document.querySelector("dd");

// Query Deleted Text Element
/** @type {HTMLModElement | null} */
const queryHTMLDeletedText = document.querySelector("del");

// Query Details Disclosure Element
/** @type {HTMLDetailsElement | null} */
const queryHTMLDetailsDisclosure = document.querySelector("details");

// Query Definition Element
/** @type {HTMLElement | null} */
const queryHTMLDefinition = document.querySelector("dfn");

// Query Dialog Element
/** @type {HTMLDialogElement | null} */
const queryHTMLDialog = document.querySelector("dialog");

// Query Content Division Element
/** @type {HTMLDivElement | null} */
const queryHTMLContentDivision = document.querySelector("div");

// Query Description List Element
/** @type {HTMLDListElement | null} */
const queryHTMLDescriptionList = document.querySelector("dl");

// Query Description Term Element
/** @type {HTMLElement | null} */
const queryHTMLDescriptionTerm = document.querySelector("dt");

// Query Emphasis Element
/** @type {HTMLElement | null} */
const queryHTMLEmphasis = document.querySelector("em");

// Query Embed External Content Element
/** @type {HTMLEmbedElement | null} */
const queryHTMLEmbedExternalContent = document.querySelector("embed");

// Query Field Set Element
/** @type {HTMLFieldSetElement | null} */
const queryHTMLFieldSet = document.querySelector("fieldset");

// Query Figure Caption Element
/** @type {HTMLElement | null} */
const queryHTMLFigureCaption = document.querySelector("figcaption");

// Query Figure with Optional Caption Element
/** @type {HTMLElement | null} */
const queryHTMLFigureWithOptionalCaption = document.querySelector("figure");

// Query Footer Element
/** @type {HTMLElement | null} */
const queryHTMLFooter = document.querySelector("footer");

// Query Form Element
/** @type {HTMLFormElement | null} */
const queryHTMLForm = document.querySelector("form");

// Query Section Heading Element
/** @type {HTMLHeadingElement | null} */
const queryHTMLSectionHeading = document.querySelector("h1");

// Query Document Metadata Header Element
/** @type {HTMLHeadElement | null} */
const queryHTMLDocumentMetadataHeader = document.querySelector("head");

// Query Header Element
/** @type {HTMLElement | null} */
const queryHTMLHeader = document.querySelector("header");

// Query Header Group Element
/** @type {HTMLElement | null} */
const queryHTMLHeadergGroup = document.querySelector("hgroup");

// Query Thematic Break Horizontal Rule Element
/** @type {HTMLHRElement | null} */
const queryHTMLThematicBreakHorizontalRule = document.querySelector("hr");

// Query HTML Document Root Element
/** @type {HTMLHtmlElement | null} */
const queryHTMLDocumentRoot = document.querySelector("html");

// Query Idiomatic Text Element
/** @type {HTMLElement | null} */
const queryHTMLIdiomaticText = document.querySelector("i");

// Query Inline Frame Element
/** @type {HTMLIFrameElement | null} */
const queryHTMLInlineFrame = document.querySelector("iframe");

// Query Image Embed Element
/** @type {HTMLImageElement | null} */
const queryHTMLImageEmbed = document.querySelector("img");

// Query Input Element
/** @type {HTMLInputElement | null} */
const queryHTMLInput = document.querySelector("input");

// Query Inserted Text Elemen
/** @type {HTMLModElement | null} */
const queryHTMLInsertedText = document.querySelector("ins");

// Query Keyboard Input Element
/** @type {HTMLElement | null} */
const queryHTMLKeyboardInput = document.querySelector("kbd");

// Query Label Element
/** @type {HTMLLabelElement | null} */
const queryHTMLLabel = document.querySelector("label");

// Query Field Set Legend Element
/** @type {HTMLLegendElement | null} */
const queryHTMLFieldSetLegend = document.querySelector("legend");

// Query List Item Element
/** @type {HTMLLIElement | null} */
const queryHTMLListItem = document.querySelector("li");

// Query External Resource Link Element
/** @type {HTMLLinkElement | null} */
const queryHTMLExternalResourceLink = document.querySelector("link");

// Query Main Element
/** @type {HTMLElement | null} */
const queryHTMLMain = document.querySelector("main");

// Query Image Map Element
/** @type {HTMLMapElement | null} */
const queryHTMLImageMap = document.querySelector("map");

// Query Mark Text Element
/** @type {HTMLElement | null} */
const queryHTMLMarkText = document.querySelector("mark");

// Query Menu Element
/** @type {HTMLMenuElement | null} */
const queryHTMLMenu = document.querySelector("menu");

// Query Metadata Element
/** @type {HTMLMetaElement | null} */
const queryHTMLMetadata = document.querySelector("meta");

// Query Meter Element
/** @type {HTMLMeterElement | null} */
const queryHTMLMeter = document.querySelector("meter");

// Query Navigation Section Element
/** @type {HTMLElement | null} */
const queryHTMLNavigationSection = document.querySelector("nav");

// Query Noscript Element
/** @type {HTMLElement | null} */
const queryHTMLNoScript = document.querySelector("noscript");

// Query External Object Element
/** @type {HTMLObjectElement | null} */
const queryHTMLExternalObject = document.querySelector("object");

// Query Ordered List Element
/** @type {HTMLOListElement | null} */
const queryHTMLOrderedList = document.querySelector("ol");

// Query Option Group Element
/** @type {HTMLOptGroupElement | null} */
const queryHTMLOptionGroup = document.querySelector("optgroup");

// Query Option Element
/** @type {HTMLOptionElement | null} */
const queryHTMLOption = document.querySelector("option");

// Query Output Element
/** @type {HTMLOutputElement | null} */
const queryHTMLOutput = document.querySelector("output");

// Query Paragraph Element
/** @type {HTMLParagraphElement | null} */
const queryHTMLParagraph = document.querySelector("p");

// Query Picture Element
/** @type {HTMLPictureElement | null} */
const queryHTMLPicture = document.querySelector("picture");

// Query Preformatted Text Element
/** @type {HTMLPreElement | null} */
const queryHTMLPreformattedText = document.querySelector("pre");

// Query Progress Indicator Element
/** @type {HTMLProgressElement | null} */
const queryHTMLProgressIndicator = document.querySelector("progress");

// Query Inline Quotation Element
/** @type {HTMLQuoteElement | null} */
const queryHTMLInlineQuotation = document.querySelector("q");

// Query Ruby Fallback Parenthesis Element
/** @type {HTMLElement | null} */
const queryHTMLRubyFallbackParenthesis = document.querySelector("rp");

// Query Ruby Text Element
/** @type {HTMLElement | null} */
const queryHTMLRubyText = document.querySelector("rt");

// Query Ruby Annotation Element
/** @type {HTMLElement | null} */
const queryHTMLRubyAnnotation = document.querySelector("ruby");

// Query Strike Through Element
/** @type {HTMLElement | null} */
const queryHTMLStrikeThrough = document.querySelector("s");

// Query Sample Output Element
/** @type {HTMLElement | null} */
const queryHTMLSampleOutput = document.querySelector("samp");

// Query Script Element
/** @type {HTMLScriptElement | null} */
const queryHTMLScript = document.querySelector("script");

// Query Generic Search Element
/** @type {HTMLElement | null} */
const queryHTMLGenericSearch = document.querySelector("search");

// Query Generic Search Element
/** @type {HTMLElement | null} */
const queryHTMLGenericSection = document.querySelector("section");

// Query Select Element
/** @type {HTMLSelectElement | null} */
const queryHTMLSelect = document.querySelector("select");

// Query Web Component Slot Element
/** @type {HTMLSlotElement | null} */
const queryHTMLWebComponentSlot = document.querySelector("slot");

// Query Side Comment Element
/** @type {HTMLElement | null} */
const queryHTMLSideComment = document.querySelector("small");

// Query Media or Image Source Element
/** @type {HTMLSourceElement | null} */
const queryHTMLMediaOrImageSource = document.querySelector("source");

// Query Content Span Element
/** @type {HTMLSpanElement | null} */
const queryHTMLContentSpan = document.querySelector("span");

// Query Strong Importance Element
/** @type {HTMLElement | null} */
const queryHTMLStrongImportance = document.querySelector("strong");

// Query Style Information Element
/** @type {HTMLStyleElement | null} */
const queryHTMLStyleInformation = document.querySelector("style");

// Query Subscript Element
/** @type {HTMLElement | null} */
const queryHTMLSubScript = document.querySelector("sub");

// Query Disclosure Summary Element
/** @type {HTMLElement | null} */
const queryHTMLDisclosureSummary = document.querySelector("summary");

// Query Superscript Element
/** @type {HTMLElement | null} */
const queryHTMLSuperScript = document.querySelector("sup");

// Query Table Element
/** @type {HTMLTableElement | null} */
const queryHTMLTable = document.querySelector("table");

// Query Table Body Element
/** @type {HTMLTableSectionElement | null} */
const queryHTMLTableBody = document.querySelector("tbody");

// Query Table Data Cell Element
/** @type {HTMLTableCellElement | null} */
const queryHTMLTableDataCell = document.querySelector("td");

// Query Content Template Element
/** @type {HTMLTemplateElement | null} */
const queryHTMLContentTemplate = document.querySelector("template");

// Query Text Area Element
/** @type {HTMLTextAreaElement | null} */
const queryHTMLTextArea = document.querySelector("textarea");

// Query Table Foot Element
/** @type {HTMLTableSectionElement | null} */
const queryHTMLTableFoot = document.querySelector("tfoot");

// Query Table Header Element
/** @type {HTMLTableCellElement | null} */
const queryHTMLTableHeader = document.querySelector("th");

// Query Table Head Element
/** @type {HTMLTableSectionElement | null} */
const queryHTMLTableHead = document.querySelector("thead");

// Query Date Time Element
/** @type {HTMLTimeElement | null} */
const queryHTMLDateTime = document.querySelector("time");

// Query Document Title Element
/** @type {HTMLTitleElement | null} */
const queryHTMLDocumentTitle  = document.querySelector("title");

// Query Table Row Element
/** @type {HTMLTableRowElement | null} */
const queryHTMLTableRow = document.querySelector("tr");

// Query Embed Text Track Element
/** @type {HTMLTrackElement | null} */
const queryHTMLEmbedTextTrack = document.querySelector("track");

// Query Unarticulated Annotation Underline Element
/** @type {HTMLElement | null} */
const queryHTMLUnarticulatedAnnotationUnderline = document.querySelector("u");

// Query Unordered List Element
/** @type {HTMLUListElement | null} */
const queryHTMLUnorderedList = document.querySelector("ul");

// Query Variable Element
/** @type {HTMLElement | null} */
const queryHTMLVariable = document.querySelector("var");

// Query Video Embed Element
/** @type {HTMLVideoElement | null} */
const queryHTMLVideoEmbed = document.querySelector("video");

// Query Line Break Opportunity Element
/** @type {HTMLElement | null} */
const queryHTMLLineBreakOpportunity = document.querySelector("wbr");
