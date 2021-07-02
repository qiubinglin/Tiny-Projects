#ifndef TRIANGEL_LINEAR_INTERPOLATION
#define TRIANGEL_LINEAR_INTERPOLATION

#include <vector>
#include "geometry.h"

std::vector<std::vector<Vec3d>> triangle_linear_interpolation(const std::vector<std::vector<Vec3d>>& src_data, size_t dest_dim);

#endif