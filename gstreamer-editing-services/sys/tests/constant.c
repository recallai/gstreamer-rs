// Generated by gir (https://github.com/gtk-rs/gir @ 3158f69)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7d95377)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 831b444)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GES_ASSET_LOADING_ASYNC);
    PRINT_CONSTANT((gint) GES_ASSET_LOADING_ERROR);
    PRINT_CONSTANT((gint) GES_ASSET_LOADING_OK);
    PRINT_CONSTANT((gint) GES_CHILDREN_IGNORE_NOTIFIES);
    PRINT_CONSTANT((gint) GES_CHILDREN_LAST);
    PRINT_CONSTANT((gint) GES_CHILDREN_UPDATE);
    PRINT_CONSTANT((gint) GES_CHILDREN_UPDATE_ALL_VALUES);
    PRINT_CONSTANT((gint) GES_CHILDREN_UPDATE_OFFSETS);
    PRINT_CONSTANT((gint) GES_EDGE_END);
    PRINT_CONSTANT((gint) GES_EDGE_NONE);
    PRINT_CONSTANT((gint) GES_EDGE_START);
    PRINT_CONSTANT((gint) GES_EDIT_MODE_NORMAL);
    PRINT_CONSTANT((gint) GES_EDIT_MODE_RIPPLE);
    PRINT_CONSTANT((gint) GES_EDIT_MODE_ROLL);
    PRINT_CONSTANT((gint) GES_EDIT_MODE_SLIDE);
    PRINT_CONSTANT((gint) GES_EDIT_MODE_TRIM);
    PRINT_CONSTANT((gint) GES_ERROR_ASSET_LOADING);
    PRINT_CONSTANT((gint) GES_ERROR_ASSET_WRONG_ID);
    PRINT_CONSTANT((gint) GES_ERROR_FORMATTER_MALFORMED_INPUT_FILE);
    PRINT_CONSTANT((gint) GES_ERROR_INVALID_EFFECT_BIN_DESCRIPTION);
    PRINT_CONSTANT((gint) GES_ERROR_INVALID_FRAME_NUMBER);
    PRINT_CONSTANT((gint) GES_ERROR_INVALID_OVERLAP_IN_TRACK);
    PRINT_CONSTANT((gint) GES_ERROR_NEGATIVE_LAYER);
    PRINT_CONSTANT((gint) GES_ERROR_NEGATIVE_TIME);
    PRINT_CONSTANT((gint) GES_ERROR_NOT_ENOUGH_INTERNAL_CONTENT);
    PRINT_CONSTANT(GES_FRAME_NUMBER_NONE);
    PRINT_CONSTANT(GES_META_DESCRIPTION);
    PRINT_CONSTANT(GES_META_FORMATTER_EXTENSION);
    PRINT_CONSTANT(GES_META_FORMATTER_MIMETYPE);
    PRINT_CONSTANT(GES_META_FORMATTER_NAME);
    PRINT_CONSTANT(GES_META_FORMATTER_RANK);
    PRINT_CONSTANT(GES_META_FORMATTER_VERSION);
    PRINT_CONSTANT(GES_META_FORMAT_VERSION);
    PRINT_CONSTANT(GES_META_MARKER_COLOR);
    PRINT_CONSTANT((guint) GES_META_READABLE);
    PRINT_CONSTANT((guint) GES_META_READ_WRITE);
    PRINT_CONSTANT(GES_META_VOLUME);
    PRINT_CONSTANT(GES_META_VOLUME_DEFAULT);
    PRINT_CONSTANT((guint) GES_META_WRITABLE);
    PRINT_CONSTANT(GES_MULTI_FILE_URI_PREFIX);
    PRINT_CONSTANT(GES_PADDING);
    PRINT_CONSTANT(GES_PADDING_LARGE);
    PRINT_CONSTANT((guint) GES_PIPELINE_MODE_PREVIEW);
    PRINT_CONSTANT((guint) GES_PIPELINE_MODE_PREVIEW_AUDIO);
    PRINT_CONSTANT((guint) GES_PIPELINE_MODE_PREVIEW_VIDEO);
    PRINT_CONSTANT((guint) GES_PIPELINE_MODE_RENDER);
    PRINT_CONSTANT((guint) GES_PIPELINE_MODE_SMART_RENDER);
    PRINT_CONSTANT((gint) GES_TEXT_HALIGN_ABSOLUTE);
    PRINT_CONSTANT((gint) GES_TEXT_HALIGN_CENTER);
    PRINT_CONSTANT((gint) GES_TEXT_HALIGN_LEFT);
    PRINT_CONSTANT((gint) GES_TEXT_HALIGN_POSITION);
    PRINT_CONSTANT((gint) GES_TEXT_HALIGN_RIGHT);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_ABSOLUTE);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_BASELINE);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_BOTTOM);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_CENTER);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_POSITION);
    PRINT_CONSTANT((gint) GES_TEXT_VALIGN_TOP);
    PRINT_CONSTANT(GES_TIMELINE_ELEMENT_NO_LAYER_PRIORITY);
    PRINT_CONSTANT((guint) GES_TRACK_TYPE_AUDIO);
    PRINT_CONSTANT((guint) GES_TRACK_TYPE_CUSTOM);
    PRINT_CONSTANT((guint) GES_TRACK_TYPE_TEXT);
    PRINT_CONSTANT((guint) GES_TRACK_TYPE_UNKNOWN);
    PRINT_CONSTANT((guint) GES_TRACK_TYPE_VIDEO);
    PRINT_CONSTANT(GES_VERSION_MAJOR);
    PRINT_CONSTANT(GES_VERSION_MICRO);
    PRINT_CONSTANT(GES_VERSION_MINOR);
    PRINT_CONSTANT(GES_VERSION_NANO);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_DBL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_DTL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_H);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_V);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_D);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_L);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_R);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_U);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BAR_WIPE_LR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BAR_WIPE_TB);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOWTIE_H);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOWTIE_V);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BC);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_LC);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_RC);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TC);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW12);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW3);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW6);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW9);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CROSSFADE);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DIAGONAL_TL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DIAGONAL_TR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FIH);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FIV);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FOH);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FOV);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_OH);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_OV);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PD);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PDBL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PDTL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PV);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_B);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_CR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_CT);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_L);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_R);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_T);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FOUR_BOX_WIPE_CI);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FOUR_BOX_WIPE_CO);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_IRIS_RECT);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_MISC_DIAGONAL_DBD);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_MISC_DIAGONAL_DD);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_NONE);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_FB);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_TBH);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_TBV);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_B);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_L);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_R);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_T);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWB);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWBL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWBR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWT);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWTL);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWTR);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_D);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_L);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_R);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_U);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_H);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_R);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_U);
    PRINT_CONSTANT((gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_V);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_CHROMA_ZONE_PLATE);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_GAMUT);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_BLACK);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_BLINK);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_BLUE);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_CHECKERS1);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_CHECKERS2);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_CHECKERS4);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_CHECKERS8);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_CIRCULAR);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_GREEN);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_RED);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_SMPTE);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_SMPTE75);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_SNOW);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_SOLID);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_PATTERN_WHITE);
    PRINT_CONSTANT((gint) GES_VIDEO_TEST_ZONE_PLATE);
    return 0;
}
