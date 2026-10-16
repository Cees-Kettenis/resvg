# Text-path positioning accuracy

The comparison uses the unchanged `inline_svg_text_paths.html` browser fixture from native_elixir_pdf_utilities. All three images use the same 420 × 300 CSS pixel canvas, 192 DPI, bundled DejaVu Sans fonts, and the existing SVG → PNG → ordinary PDF image workflow. The old and fixed runs use an identical Chrome reference image.

- Old: Resvg fork `fd5af37ef2f17d649da846bda7004733b4ff9cc4`. Unmodified upstream `978775b5f2d31f35b370e0ac4059038be117101c` produces the same comparison statistics for this fixture.
- Fixed: branch `fix-text-path-accuracy`, changing the base arc-length accuracy from 0.5 to 0.001 before the existing transform-scale adjustment.
- Chrome: Chromium 152.0.7977.82, rendering the authored SVG with the same fonts and intended viewport into the PNG comparison reference.
- Whole-sample differing pixels at channel tolerance 12/255: 1.0224206% before, 0.6698413% after. The original 1% limit and original authored fixture were unchanged.

`source.html` is the authored fixture. `source.svg` extracts its SVG and supplies the document's DejaVu Sans font family and explicit viewport dimensions. The standalone SVG excludes the HTML's 12px page margin. `old.png`, `new.png`, and `chrome.png` retain that margin and are unscaled crops of the saved browser comparison output. `old-vs-new.png` only adds column labels.

## Upstream snapshot review

The 42 upstream snapshots affected by the accuracy change were rendered with the test suite's sorted bundled font files, family mappings, and 300px output width. The baseline reproduces the original snapshots within the suite's one-channel-value tolerance.

For an independent precision control, the same SVGs were also parsed with 0.000001 arc-length accuracy. All 661 glyph IDs and their order are unchanged. Every affected case has lower mean image error and lower maximum glyph-origin error against that tighter control after the fix. The maximum glyph-origin deviation across the cases drops from 0.552105 to 0.001184 SVG units. `convergence-review.json` contains the measurements per fixture.

This control checks numerical convergence; it is not a claim that every upstream fixture matches Chrome exactly. Original upstream PNGs remain available at upstream commit `978775b5f2d31f35b370e0ac4059038be117101c` and were retained locally before updating the branch's expected snapshots. No Chrome reference was replaced.
