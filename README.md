# egui_fancy_knob

Feature-rich circular knob widget, originally forked from egui_knob.

## Basic features (from egui_knob)

* Adjustable size, font size, and stroke width.
* Customizable colors for the knob, indicator and text.
* Label positions (Top, Bottom, Left, Right).
* Label formatting.
* Two styles: Wiper and Dot.

Credit where credit is due: https://github.com/obsqrbtz/egui_knob

## Improvements upon egui_knob

* Passing value + setter instead of a mutable value. This allows for more flexible state management patterns (e.g. Model/View/Intent which I've found to work well with egui).
* Support for neutral position / reset.
* Support for fine dragging.
* Support for disabled/unmodifiable knob.
* Knob position is rendered symmetrically.
* Visual indication of knob being dragged (size increase + color change).
* Helper wrapper that calls an on_release callback when the mouse is lifted after dragging.
* Fixes a bug where labels were inconsistently positioned (especially for large ranges of values).
* Adds an optional extra_debug feature which renders bounding boxes around the knob, helpful for debugging.
