// Copyright 2023 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{render_extra, render_extra_with_scale, render_node};

#[test]
fn group_with_only_transform() {
    assert_eq!(render_extra("extra/group-with-only-transform"), 0);
}

#[test]
fn subpixel_rect_position() {
    assert_eq!(render_extra("extra/subpixel-rect-position"), 0);
}

#[test]
fn transformed_rect() {
    assert_eq!(render_extra("extra/transformed-rect"), 0);
}

#[test]
fn hidden_element() {
    assert_eq!(render_extra("extra/hidden-element"), 0);
}

#[test]
fn simple_stroke() {
    assert_eq!(render_extra("extra/simple-stroke"), 0);
}

#[test]
fn fill_and_stroke() {
    assert_eq!(render_extra("extra/fill-and-stroke"), 0);
}

#[test]
fn paint_order_stroke() {
    assert_eq!(render_extra("extra/paint-order=stroke"), 0);
}

#[test]
fn stroke_linecap_square() {
    assert_eq!(render_extra("extra/stroke-linecap=square"), 0);
}

#[test]
fn miter_join_with_acute_angle() {
    assert_eq!(render_extra("extra/miter-join-with-acute-angle"), 0);
}

#[test]
fn horizontal_line() {
    assert_eq!(render_extra("extra/horizontal-line"), 0);
}

#[test]
fn horizontal_line_no_stroke() {
    assert_eq!(render_extra("extra/horizontal-line-no-stroke"), 0);
}

#[test]
fn filter_region_precision() {
    assert_eq!(
        render_extra_with_scale("extra/filter-region-precision", 10.0),
        0
    );
}

#[test]
fn translate_outside_viewbox() {
    assert_eq!(render_extra("extra/translate-outside-viewbox"), 0);
}

#[test]
fn render_node_filter_on_empty_group() {
    assert_eq!(render_node("extra/filter-on-empty-group", "g1"), 0);
}

#[test]
fn render_node_filter_with_transform_on_shape() {
    assert_eq!(render_node("extra/filter-with-transform-on-shape", "g1"), 0);
}

#[test]
fn fractional_pattern_fill() {
    assert_eq!(render_extra_with_scale("extra/fractional-pattern", 2.0), 0);
}

#[test]
fn fractional_pattern_stroke() {
    assert_eq!(
        render_extra_with_scale("extra/fractional-pattern-stroke", 2.0),
        0
    );
}

#[test]
fn fractional_pattern_horizontal_stripes() {
    assert_eq!(
        render_extra_with_scale("extra/fractional-pattern-horizontal-stripes", 2.0),
        0
    );
}

#[test]
fn fractional_pattern_transforms() {
    assert_eq!(
        render_extra_with_scale("extra/fractional-pattern-transforms", 1.5),
        0
    );
}

#[test]
fn linear_rgb_gradient() {
    assert_eq!(render_extra_with_scale("extra/linear-rgb-gradient", 2.0), 0);
}

#[test]
fn linear_rgb_mask() {
    assert_eq!(render_extra_with_scale("extra/linear-rgb-mask", 2.0), 0);
}

#[test]
fn linear_rgb_gradient_radial() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-radial", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_colors() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-colors", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_alpha() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-alpha", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_hard_stops() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-hard-stops", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_stroke_reflect() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-stroke-reflect", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_inheritance() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-inheritance", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_href() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-href", 2.0),
        0
    );
}

#[test]
fn linear_rgb_mask_colors_alpha() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-mask-colors-alpha", 2.0),
        0
    );
}

#[test]
fn linear_rgb_mask_alpha() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-mask-alpha", 2.0),
        0
    );
}

#[test]
fn linear_rgb_mask_gradient() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-mask-gradient", 2.0),
        0
    );
}

#[test]
fn linear_rgb_gradient_repeat() {
    assert_eq!(
        render_extra_with_scale("extra/linear-rgb-gradient-repeat", 2.0),
        0
    );
}

#[test]
fn marker_degenerate_tangent() {
    assert_eq!(
        render_extra_with_scale("extra/marker-degenerate-tangent", 2.0),
        0
    );
}

#[test]
fn marker_closed_join() {
    assert_eq!(render_extra_with_scale("extra/marker-closed-join", 2.0), 0);
}

#[test]
fn marker_closed_zero_line() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-zero-line", 2.0),
        0
    );
}

#[test]
fn marker_degenerate_start_reverse() {
    assert_eq!(
        render_extra_with_scale("extra/marker-degenerate-start-reverse", 2.0),
        0
    );
}

#[test]
fn marker_closed_start_reverse() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-start-reverse", 2.0),
        0
    );
}

#[test]
fn marker_degenerate_fixed_angle() {
    assert_eq!(
        render_extra_with_scale("extra/marker-degenerate-fixed-angle", 2.0),
        0
    );
}

#[test]
fn marker_closed_subpaths() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-subpaths", 2.0),
        0
    );
}

#[test]
fn marker_closed_polygon() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-polygon", 2.0),
        0
    );
}

#[test]
fn marker_degenerate_controls() {
    assert_eq!(
        render_extra_with_scale("extra/marker-degenerate-controls", 2.0),
        0
    );
}

#[test]
fn marker_closed_curve_mid() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-curve-mid", 2.0),
        0
    );
}

#[test]
fn marker_closed_zero_line_mid() {
    assert_eq!(
        render_extra_with_scale("extra/marker-closed-zero-line-mid", 2.0),
        0
    );
}

#[test]
fn tiny_raster_horizontal() {
    assert_eq!(render_extra("extra/tiny-raster-horizontal"), 0);
}

#[test]
fn tiny_raster_horizontal_two_rows() {
    assert_eq!(render_extra("extra/tiny-raster-horizontal-two-rows"), 0);
}

#[test]
fn tiny_raster_vertical() {
    assert_eq!(render_extra("extra/tiny-raster-vertical"), 0);
}

#[test]
fn tiny_raster_vertical_two_columns() {
    assert_eq!(render_extra("extra/tiny-raster-vertical-two-columns"), 0);
}

#[test]
fn tiny_raster_single_pixel() {
    assert_eq!(render_extra("extra/tiny-raster-single-pixel"), 0);
}

#[test]
fn tiny_raster_horizontal_density_two() {
    assert_eq!(
        render_extra_with_scale("extra/tiny-raster-horizontal-density-two", 2.0),
        0
    );
}

#[test]
fn tiny_raster_preserves_stroke_paint_and_destination() {
    let bodies = [
        r#"<path d="M-8 .5H80M.5 -8V80" stroke="red" stroke-opacity=".5" fill="none"/>"#,
        r#"<path d="M-8 .5H80M.5 -8V80" stroke="red" stroke-width=".5" stroke-dasharray="3 2" fill="none"/>"#,
        r#"<path d="M-8 .5Q32 .5 80 .5M.5 -8C.5 0 .5 64 .5 80" stroke="red" stroke-linecap="round" fill="none"/>"#,
        r#"<defs><linearGradient id="g" gradientUnits="userSpaceOnUse" x2="64"><stop stop-color="red"/><stop offset="1" stop-color="blue"/></linearGradient></defs><path d="M-8 .5H80M.5 -8V80" stroke="url(#g)" fill="none"/>"#,
        r#"<defs><pattern id="p" patternUnits="userSpaceOnUse" width="4" height="4"><rect width="2" height="4" fill="red"/><rect x="2" width="2" height="4" fill="blue"/></pattern></defs><path d="M-8 .5H80M.5 -8V80" stroke="url(#p)" fill="none"/>"#,
    ];

    for (index, body) in bodies.iter().enumerate() {
        let source = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64">{body}</svg>"#
        );
        let tree = usvg::Tree::from_str(&source, &usvg::Options::default()).unwrap();
        for (width, height) in [(64, 1), (64, 2), (1, 64), (2, 64), (1, 1), (2, 2)] {
            for alpha in [0, 128, 255] {
                let background = tiny_skia::Color::from_rgba8(0, 100, 200, alpha);
                let mut actual = tiny_skia::Pixmap::new(width, height).unwrap();
                actual.fill(background);
                let mut reference = tiny_skia::Pixmap::new(width.max(8), height.max(8)).unwrap();
                reference.fill(background);
                for (tx, ty) in [(0.0, 0.0), (0.25, 0.25)] {
                    let transform = tiny_skia::Transform::from_translate(tx, ty);
                    resvg::render(&tree, transform, &mut actual.as_mut());
                    resvg::render(&tree, transform, &mut reference.as_mut());
                    let expected = reference
                        .clone_rect(tiny_skia::IntRect::from_xywh(0, 0, width, height).unwrap())
                        .unwrap();
                    assert_eq!(
                        actual, expected,
                        "paint={index}, canvas={width}x{height}, alpha={alpha}, offset=({tx}, {ty})"
                    );
                }
            }
        }
    }
}
