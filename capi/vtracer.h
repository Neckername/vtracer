#ifndef VTRACER_H
#define VTRACER_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>

/**
 * Convert an image file to an SVG using VTracer's default configuration.
 *
 * @param input_path  Path to the input raster image.
 * @param output_path Path where the SVG will be written.
 * @return true on success, false on failure.
 */
bool vtracer_convert_image_to_svg_default(const char *input_path,
                                          const char *output_path);

#ifdef __cplusplus
}
#endif
#endif /* VTRACER_H */
