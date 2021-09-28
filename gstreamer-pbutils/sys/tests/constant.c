// Generated by gir (https://github.com/gtk-rs/gir @ 2e986b5dc4c5)
// from gir-files (https://github.com/gtk-rs/gir-files @ b827978e7d18)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 35f2f148e071)
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
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT);
    PRINT_CONSTANT((gint) GST_AUDIO_VISUALIZER_SHADER_NONE);
    PRINT_CONSTANT((gint) GST_DISCOVERER_BUSY);
    PRINT_CONSTANT((gint) GST_DISCOVERER_ERROR);
    PRINT_CONSTANT((gint) GST_DISCOVERER_MISSING_PLUGINS);
    PRINT_CONSTANT((gint) GST_DISCOVERER_OK);
    PRINT_CONSTANT((guint) GST_DISCOVERER_SERIALIZE_ALL);
    PRINT_CONSTANT((guint) GST_DISCOVERER_SERIALIZE_BASIC);
    PRINT_CONSTANT((guint) GST_DISCOVERER_SERIALIZE_CAPS);
    PRINT_CONSTANT((guint) GST_DISCOVERER_SERIALIZE_MISC);
    PRINT_CONSTANT((guint) GST_DISCOVERER_SERIALIZE_TAGS);
    PRINT_CONSTANT((gint) GST_DISCOVERER_TIMEOUT);
    PRINT_CONSTANT((gint) GST_DISCOVERER_URI_INVALID);
    PRINT_CONSTANT(GST_ENCODING_CATEGORY_CAPTURE);
    PRINT_CONSTANT(GST_ENCODING_CATEGORY_DEVICE);
    PRINT_CONSTANT(GST_ENCODING_CATEGORY_FILE_EXTENSION);
    PRINT_CONSTANT(GST_ENCODING_CATEGORY_ONLINE_SERVICE);
    PRINT_CONSTANT(GST_ENCODING_CATEGORY_STORAGE_EDITING);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_CRASHED);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_ERROR);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_HELPER_MISSING);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_INSTALL_IN_PROGRESS);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_INTERNAL_FAILURE);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_INVALID);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_NOT_FOUND);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_PARTIAL_SUCCESS);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_STARTED_OK);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_SUCCESS);
    PRINT_CONSTANT((gint) GST_INSTALL_PLUGINS_USER_ABORT);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_AUDIO);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_CONTAINER);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_GENERIC);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_IMAGE);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_SUBTITLE);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_TAG);
    PRINT_CONSTANT((guint) GST_PBUTILS_CAPS_DESCRIPTION_FLAG_VIDEO);
    return 0;
}
