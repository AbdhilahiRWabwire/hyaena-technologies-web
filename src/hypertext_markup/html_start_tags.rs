#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Markup Language Element Tag Definition
pub type HTMLStartTag = &'static str;

// Hypertext Markup Language Element Start Tag
pub const HTML_ANCHOR_START_TAG: HTMLStartTag = "<a>";
pub const HTML_ABBREVIATION_START_TAG: HTMLStartTag = "<abbr>";
pub const HTML_CONTACT_ADDRESS_START_TAG: HTMLStartTag = "<address>";
pub const HTML_IMAGE_MAP_AREA_START_TAG: HTMLStartTag = "<area>";
pub const HTML_ARTICLE_START_TAG: HTMLStartTag = "<article>";
pub const HTML_ASIDE_CONTENT_START_TAG: HTMLStartTag = "<aside>";
pub const HTML_AUDIO_START_TAG: HTMLStartTag = "<audio>";
pub const HTML_BRING_ATTENTION_TO_START_TAG: HTMLStartTag = "<b>";
pub const HTML_BASE_URL_START_TAG: HTMLStartTag = "<base>";
pub const HTML_BIDIRECTIONAL_ISOLATE_START_TAG: HTMLStartTag = "<bdi>";
pub const HTML_BIDIRECTIONAL_TEXT_OVERRIDE_START_TAG: HTMLStartTag = "<bdo>";
pub const HTML_BLOCK_QUOTATION_START_TAG: HTMLStartTag = "<blockquote>";
pub const HTML_DOCUMENT_BODY_START_TAG: HTMLStartTag = "<body>";
pub const HTML_LINE_BREAK_START_TAG: HTMLStartTag = "<br>";
pub const HTML_BUTTON_START_TAG: HTMLStartTag = "<button>";
pub const HTML_GRAPHICS_CANVAS_START_TAG: HTMLStartTag = "<canvas>";
pub const HTML_TABLE_CAPTION_START_TAG: HTMLStartTag = "<caption>";
pub const HTML_CITATION_START_TAG: HTMLStartTag = "<cite>";
pub const HTML_INLINE_CODE_START_TAG: HTMLStartTag = "<code>";
pub const HTML_TABLE_COLUMN_START_TAG: HTMLStartTag = "<col>";
pub const HTML_TABLE_COLUMN_GROUP_START_TAG: HTMLStartTag = "<colgroup>";
pub const HTML_DATA_START_TAG: HTMLStartTag = "<data>";
pub const HTML_DATA_LIST_START_TAG: HTMLStartTag = "<datalist>";
pub const HTML_DESCRIPTION_DETAILS_START_TAG: HTMLStartTag = "<dd>";
pub const HTML_DELETED_TEXT_START_TAG: HTMLStartTag = "<del>";
pub const HTML_DETAILS_DISCLOSURE_START_TAG: HTMLStartTag = "<details>";
pub const HTML_DEFINITION_START_TAG: HTMLStartTag = "<dfn>";
pub const HTML_DIALOGUE_START_TAG: HTMLStartTag = "<dialog>";
pub const HTML_CONTENT_DIVISION_START_TAG: HTMLStartTag = "<div>";
pub const HTML_DESCRIPTION_LIST_START_TAG: HTMLStartTag = "<dl>";
pub const HTML_DESCRIPTION_TERM_START_TAG: HTMLStartTag = "<dt>";
pub const HTML_EMPHASIS_START_TAG: HTMLStartTag = "<em>";
pub const HTML_EMBEDED_EXTERNAL_CONTENT_START_TAG: HTMLStartTag = "<embed>";
pub const HTML_FIELD_SET_START_TAG: HTMLStartTag = "<fieldset>";
pub const HTML_FIGURE_CAPTION_START_TAG: HTMLStartTag = "<figcaption>";
pub const HTML_FIGURE_START_TAG: HTMLStartTag = "<figure>";
pub const HTML_FOOTER_START_TAG: HTMLStartTag = "<footer>";
pub const HTML_FORM_START_TAG: HTMLStartTag = "<form>";
pub const HTML_SECTION_HEADING_ONE_START_TAG: HTMLStartTag = "<h1>";
pub const HTML_SECTION_HEADING_TWO_START_TAG: HTMLStartTag = "<h2>";
pub const HTML_SECTION_HEADING_THREE_START_TAG: HTMLStartTag = "<h3>";
pub const HTML_SECTION_HEADING_FOUR_START_TAG: HTMLStartTag = "<h4>";
pub const HTML_SECTION_HEADING_FIVE_START_TAG: HTMLStartTag = "<h5>";
pub const HTML_SECTION_HEADING_SIX_START_TAG: HTMLStartTag = "<h6>";
pub const HTML_DOCUMENT_METADATA_HEADER_START_TAG: HTMLStartTag = "<head>";
pub const HTML_HEADER_START_TAG: HTMLStartTag = "<header>";
pub const HTML_HEADING_GROUP_START_TAG: HTMLStartTag = "<hgroup>";
pub const HTML_THEMATIC_BREAK_HORIZONTAL_RULE_START_TAG: HTMLStartTag = "<hr>";
pub const HTML_DOCUMENT_ROOT_START_TAG: HTMLStartTag = "<html>";
pub const HTML_IDIOMATIC_TEXT_START_TAG: HTMLStartTag = "<i>";
pub const HTML_INLINE_FRAME_START_TAG: HTMLStartTag = "<iframe>";
pub const HTML_IMAGE_EMBED_START_TAG: HTMLStartTag = "<img>";
pub const HTML_INPUT_START_TAG: HTMLStartTag = "<input>";
pub const HTML_INSERTED_TEXT_START_TAG: HTMLStartTag = "<ins>";
pub const HTML_KEYBOARD_INSET_START_TAG: HTMLStartTag = "<kbd>";
pub const HTML_LABEL_START_TAG: HTMLStartTag = "<label>";
pub const HTML_FIELD_SET_LEGEND_START_TAG: HTMLStartTag = "<legend>";
pub const HTML_LIST_ITEM_START_TAG: HTMLStartTag = "<li>";
pub const HTML_EXTERNAL_RESOURCE_LINK_START_TAG: HTMLStartTag = "<link>";
pub const HTML_MAIN_START_TAG: HTMLStartTag = "<main>";
pub const HTML_IMAGE_MAP_START_TAG: HTMLStartTag = "<map>";
pub const HTML_MARK_TEXT_START_TAG: HTMLStartTag = "<mark>";
pub const HTML_MENU_START_TAG: HTMLStartTag = "<menu>";
pub const HTML_METADATA_START_TAG: HTMLStartTag = "<meta>";
pub const HTML_METER_START_TAG: HTMLStartTag = "<meter>";
pub const HTML_NAVIGATION_SECTION_START_TAG: HTMLStartTag = "<nav>";
pub const HTML_NO_SCRIPT_START_TAG: HTMLStartTag = "<noscript>";
pub const HTML_EXTERNAL_OBJECT_START_TAG: HTMLStartTag = "<object>";
pub const HTML_ORDERED_LIST_START_TAG: HTMLStartTag = "<ol>";
pub const HTML_OPTION_GROUP_START_TAG: HTMLStartTag = "<optgroup>";
pub const HTML_OPTION_START_TAG: HTMLStartTag = "<option>";
pub const HTML_OUTPUT_START_TAG: HTMLStartTag = "<output>";
pub const HTML_PARAGRAPH_START_TAG: HTMLStartTag = "<p>";
pub const HTML_PICTURE_START_TAG: HTMLStartTag = "<picture>";
pub const HTML_PREFORMATED_TEXT_START_TAG: HTMLStartTag = "<pre>";
pub const HTML_PROGRESS_INDIDCATOR_START_TAG: HTMLStartTag = "<progress>";
pub const HTML_INLINE_QUOTATION_START_TAG: HTMLStartTag = "<q>";
pub const HTML_RUBY_FALLBACK_PARENTHESIS_START_TAG: HTMLStartTag = "<rp>";
pub const HTML_RUBY_TEXT_START_TAG: HTMLStartTag = "<rt>";
pub const HTML_RUBY_ANNOTATION_START_TAG: HTMLStartTag = "<ruby>";
pub const HTML_STRIKE_THROUGH_START_TAG: HTMLStartTag = "<s>";
pub const HTML_SAMPLE_OUTPUT_START_TAG: HTMLStartTag = "<samp>";
pub const HTML_SCRIPT_START_TAG: HTMLStartTag = "<script>";
pub const HTML_GENERIC_SEARCH_START_TAG: HTMLStartTag = "<search>";
pub const HTML_GENERIC_SECTION_START_TAG: HTMLStartTag = "<section>";
pub const HTML_SELECT_START_TAG: HTMLStartTag = "<select>";
pub const HTML_WEB_COMPONENT_SLOT_START_TAG: HTMLStartTag = "<slot>";
pub const HTML_SIDE_COMMENT_SMALL_PRINT_START_TAG: HTMLStartTag = "<small>";
pub const HTML_MEDIA_IMAGE_SOURCE_START_TAG: HTMLStartTag = "<source>";
pub const HTML_CONTENT_SPAN_START_TAG: HTMLStartTag = "<span>";
pub const HTML_STRONG_IMPORTANCE_START_TAG: HTMLStartTag = "<strong>";
pub const HTML_STYLE_INFORMATION_START_TAG: HTMLStartTag = "<style>";
pub const HTML_SUBSCRIPT_START_TAG: HTMLStartTag = "<sub>";
pub const HTML_DISCLOSURE_SUMMARY_START_TAG: HTMLStartTag = "<summary>";
pub const HTML_SUPERSCRIPT_START_TAG: HTMLStartTag = "<sup>";
pub const HTML_TABLE_START_TAG: HTMLStartTag = "<table>";
pub const HTML_TABLE_BODY_START_TAG: HTMLStartTag = "<tbody>";
pub const HTML_TABLE_DATA_CELL_START_TAG: HTMLStartTag = "<td>";
pub const HTML_CONTENT_TEMPLATE_START_TAG: HTMLStartTag = "<template>";
pub const HTML_TEXT_AREA_START_TAG: HTMLStartTag = "<textarea>";
pub const HTML_TABLE_FOOTER_START_TAG: HTMLStartTag = "<tfoot>";
pub const HTML_TABLE_HEADER_START_TAG: HTMLStartTag = "<th>";
pub const HTML_TABLE_HEAD_START_TAG: HTMLStartTag = "<thead>";
pub const HTML_DATE_TIME_START_TAG: HTMLStartTag = "<time>";
pub const HTML_DOCUMENT_TITLE_START_TAG: HTMLStartTag = "<title>";
pub const HTML_TABLE_ROW_START_TAG: HTMLStartTag = "<tr>";
pub const HTML_EMBED_TEXT_TRACK_START_TAG: HTMLStartTag = "<track>";
pub const HTML_UNARTICULATED_ANNOTATION_START_TAG: HTMLStartTag = "<u>";
pub const HTML_UNORDERED_LIST_START_TAG: HTMLStartTag = "<ul>";
pub const HTML_VARIABLE_START_TAG: HTMLStartTag = "<var>";
pub const HTML_VIDEO_EMBED_START_TAG: HTMLStartTag = "<video>";
pub const HTML_LINE_WORD_BREAK_OPPORTUNITY_START_TAG: HTMLStartTag = "<wbr>";

// Hypertext Markup Language Start Tag Vector
pub fn html_start_tags() -> Vec<HTMLStartTag> {
    let hypertext_markup_start_tags: Vec<HTMLStartTag> = Vec::from([
        HTML_ANCHOR_START_TAG,
        HTML_ABBREVIATION_START_TAG,
        HTML_CONTACT_ADDRESS_START_TAG,
        HTML_IMAGE_MAP_AREA_START_TAG,
        HTML_ARTICLE_START_TAG,
        HTML_ASIDE_CONTENT_START_TAG,
        HTML_AUDIO_START_TAG,
        HTML_BRING_ATTENTION_TO_START_TAG,
        HTML_BASE_URL_START_TAG,
        HTML_BIDIRECTIONAL_ISOLATE_START_TAG,
        HTML_BIDIRECTIONAL_TEXT_OVERRIDE_START_TAG,
        HTML_BLOCK_QUOTATION_START_TAG,
        HTML_DOCUMENT_BODY_START_TAG,
        HTML_LINE_BREAK_START_TAG,
        HTML_BUTTON_START_TAG,
        HTML_GRAPHICS_CANVAS_START_TAG,
        HTML_TABLE_CAPTION_START_TAG,
        HTML_CITATION_START_TAG,
        HTML_INLINE_CODE_START_TAG,
        HTML_TABLE_COLUMN_START_TAG,
        HTML_TABLE_COLUMN_GROUP_START_TAG,
        HTML_DATA_START_TAG,
        HTML_DATA_LIST_START_TAG,
        HTML_DESCRIPTION_DETAILS_START_TAG,
        HTML_DELETED_TEXT_START_TAG,
        HTML_DETAILS_DISCLOSURE_START_TAG,
        HTML_DEFINITION_START_TAG,
        HTML_DIALOGUE_START_TAG,
        HTML_CONTENT_DIVISION_START_TAG,
        HTML_DESCRIPTION_LIST_START_TAG,
        HTML_DESCRIPTION_TERM_START_TAG,
        HTML_EMPHASIS_START_TAG,
        HTML_EMBEDED_EXTERNAL_CONTENT_START_TAG,
        HTML_FIELD_SET_START_TAG,
        HTML_FIGURE_CAPTION_START_TAG,
        HTML_FIGURE_START_TAG,
        HTML_FOOTER_START_TAG,
        HTML_FORM_START_TAG,
        HTML_SECTION_HEADING_ONE_START_TAG,
        HTML_SECTION_HEADING_TWO_START_TAG,
        HTML_SECTION_HEADING_THREE_START_TAG,
        HTML_SECTION_HEADING_FOUR_START_TAG,
        HTML_SECTION_HEADING_FIVE_START_TAG,
        HTML_SECTION_HEADING_SIX_START_TAG,
        HTML_DOCUMENT_METADATA_HEADER_START_TAG,
        HTML_HEADER_START_TAG,
        HTML_HEADING_GROUP_START_TAG,
        HTML_THEMATIC_BREAK_HORIZONTAL_RULE_START_TAG,
        HTML_DOCUMENT_ROOT_START_TAG,
        HTML_IDIOMATIC_TEXT_START_TAG,
        HTML_INLINE_FRAME_START_TAG,
        HTML_IMAGE_EMBED_START_TAG,
        HTML_INPUT_START_TAG,
        HTML_INSERTED_TEXT_START_TAG,
        HTML_KEYBOARD_INSET_START_TAG,
        HTML_LABEL_START_TAG,
        HTML_FIELD_SET_LEGEND_START_TAG,
        HTML_LIST_ITEM_START_TAG,
        HTML_EXTERNAL_RESOURCE_LINK_START_TAG,
        HTML_MAIN_START_TAG,
        HTML_IMAGE_MAP_START_TAG,
        HTML_MARK_TEXT_START_TAG,
        HTML_MENU_START_TAG,
        HTML_METADATA_START_TAG,
        HTML_METER_START_TAG,
        HTML_NAVIGATION_SECTION_START_TAG,
        HTML_NO_SCRIPT_START_TAG,
        HTML_EXTERNAL_OBJECT_START_TAG,
        HTML_ORDERED_LIST_START_TAG,
        HTML_OPTION_GROUP_START_TAG,
        HTML_OPTION_START_TAG,
        HTML_OUTPUT_START_TAG,
        HTML_PARAGRAPH_START_TAG,
        HTML_PICTURE_START_TAG,
        HTML_PREFORMATED_TEXT_START_TAG,
        HTML_PROGRESS_INDIDCATOR_START_TAG,
        HTML_INLINE_QUOTATION_START_TAG,
        HTML_RUBY_FALLBACK_PARENTHESIS_START_TAG,
        HTML_RUBY_TEXT_START_TAG,
        HTML_RUBY_ANNOTATION_START_TAG,
        HTML_STRIKE_THROUGH_START_TAG,
        HTML_SAMPLE_OUTPUT_START_TAG,
        HTML_SCRIPT_START_TAG,
        HTML_GENERIC_SEARCH_START_TAG,
        HTML_GENERIC_SECTION_START_TAG,
        HTML_SELECT_START_TAG,
        HTML_WEB_COMPONENT_SLOT_START_TAG,
        HTML_SIDE_COMMENT_SMALL_PRINT_START_TAG,
        HTML_MEDIA_IMAGE_SOURCE_START_TAG,
        HTML_CONTENT_SPAN_START_TAG,
        HTML_STRONG_IMPORTANCE_START_TAG,
        HTML_STYLE_INFORMATION_START_TAG,
        HTML_SUBSCRIPT_START_TAG,
        HTML_DISCLOSURE_SUMMARY_START_TAG,
        HTML_SUPERSCRIPT_START_TAG,
        HTML_TABLE_START_TAG,
        HTML_TABLE_BODY_START_TAG,
        HTML_TABLE_DATA_CELL_START_TAG,
        HTML_CONTENT_TEMPLATE_START_TAG,
        HTML_TEXT_AREA_START_TAG,
        HTML_TABLE_FOOTER_START_TAG,
        HTML_TABLE_HEADER_START_TAG,
        HTML_TABLE_HEAD_START_TAG,
        HTML_DATE_TIME_START_TAG,
        HTML_DOCUMENT_TITLE_START_TAG,
        HTML_TABLE_ROW_START_TAG,
        HTML_EMBED_TEXT_TRACK_START_TAG,
        HTML_UNARTICULATED_ANNOTATION_START_TAG,
        HTML_UNORDERED_LIST_START_TAG,
        HTML_VARIABLE_START_TAG,
        HTML_VIDEO_EMBED_START_TAG,
        HTML_LINE_WORD_BREAK_OPPORTUNITY_START_TAG,
    ]);

    return hypertext_markup_start_tags;
}
