#include "triangle_linear_interpolation.h"

#include <limits.h>
#include <algorithm>
#include <stdexcept>
#include <unordered_map>
#include <math.h>

const double EPSILON = 1e-6;

double cross(double x1, double y1, double x2, double y2) {
    return x1 * y2 - x2 * y1;
}

void triangle_linear_interpolation(Vec3d p1, Vec3d p2, Vec3d p3, Vec3d& out) {
    double u = 0, v = 0, w = 0;
    u = ((p2.y - p3.y) * (out.x - p3.x) + (p3.x - p2.x) * (out.y - p3.y)) / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));
    v = ((p3.y - p1.y) * (out.x - p3.x) + (p1.x - p3.x) * (out.y - p3.y)) / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));
    w = 1 - u - v;

    out.z = u * p1.z + v * p2.z + w * p3.z;
}

std::vector<std::vector<Vec3d>> triangle_linear_interpolation(const std::vector<std::vector<Vec3d>>& src_data, size_t dest_dim) {
    if (dest_dim == 0) {
        throw std::runtime_error("divided by zero");
    }

    auto src_x_dim = src_data.size();
    if (src_x_dim <= 1) {
        throw std::runtime_error("x dim == 1 or empty src data");
    }
    auto src_y_dim = src_data[0].size();
    if (src_y_dim <= 1) {
        throw std::runtime_error("y dim == 1 or empty src data");
    }

    if (dest_dim < src_x_dim || dest_dim < src_y_dim) {
        throw std::runtime_error("dest dim < src dim");
    }

    auto min_x = src_data[0][0].x;
    auto min_y = src_data[0][0].y;
    auto max_x = src_data[src_x_dim][src_y_dim].x;
    auto max_y = src_data[src_x_dim][src_y_dim].y;
    auto dest_gap_x = (max_x - min_x) / dest_dim;
    auto dest_gap_y = (max_y - min_y) / dest_dim;

    std::vector<std::vector<Vec3d>> dest_data(dest_dim, std::vector<Vec3d>(dest_dim, Vec3d()));
    for (size_t i = 0; i < dest_dim; ++i) {
        for (size_t j = 0; j < dest_dim; ++j) {
            double x_len = static_cast<double>(i) / dest_dim;
            double src_x_idx = x_len * src_x_dim;
            double y_len = static_cast<double>(j) / dest_dim;
            double src_y_idx = y_len * src_y_dim;

            int bottom_ix = floor(src_x_idx), top_ix = ceil(src_x_idx);
            int bottom_iy = floor(src_y_idx), top_iy = ceil(src_y_idx);

            Vec3d& dest = dest_data[i][j];
            dest.x = dest_gap_x * i;
            dest.y = dest_gap_y * j;

            double x1 = src_data[bottom_ix][bottom_iy].x;
            double y1 = src_data[bottom_ix][bottom_iy].y;
            double x2 = src_data[top_ix][bottom_iy].x;
            double y2 = src_data[top_ix][bottom_iy].y;
            double x3 = src_data[top_ix][top_iy].x;
            double y3 = src_data[top_ix][top_iy].y;
            double x4 = src_data[bottom_ix][top_iy].x;
            double y4 = src_data[bottom_ix][top_iy].y;
            if ((cross(dest.x - x1, dest.y - y1, x3 - x1, y3 - y1) >= 0)) {
                triangle_linear_interpolation(src_data[bottom_ix][bottom_iy], src_data[top_ix][bottom_iy], src_data[top_ix][top_iy], dest);
            } else {
                triangle_linear_interpolation(src_data[bottom_ix][bottom_iy], src_data[bottom_ix][top_iy], src_data[top_ix][top_iy], dest);
            }
        }
    }
}