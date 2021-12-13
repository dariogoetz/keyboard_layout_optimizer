(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../../config/evaluation_parameters.yml":
/*!**************************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/config/evaluation_parameters.yml ***!
  \**************************************************************************************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"metrics:\\n  # layour metrics\\n  shortcut_keys:\\n    enabled: true\\n    weight: 0.35\\n    normalization:\\n      type: fixed\\n      value: 1.0\\n    params:\\n      shortcut_chars: cvxz\\n      cost: 1.0\\n\\n  asymmetric_keys:\\n    enabled: true\\n    weight: 5.0\\n    normalization:\\n      type: fixed\\n      value: 1.0\\n    params:\\n      similar_letters:\\n        - [\\\"auo\\\", \\\"äüö\\\"]\\n        - [\\\"auo\\\", \\\"äüö\\\"]\\n        - [\\\"gbd\\\", \\\"kpt\\\"]\\n        # - [\\\"gbdw\\\", \\\"kptf\\\"]\\n        # - [\\\"sfdn\\\", \\\"tpbm\\\"]\\n\\n  # unigram metrics\\n  finger_balance:\\n    enabled: true\\n    weight: 69.0\\n    normalization:\\n      type: fixed\\n      value: 1.0\\n    params:\\n      intended_loads:\\n        [Left, Pinky]: 1.0\\n        [Left, Ring]: 1.6\\n        [Left, Middle]: 2.0\\n        [Left, Pointer]: 2.0\\n        [Left, Thumb]: 2.0\\n        [Right, Thumb]: 2.0\\n        [Right, Pointer]: 2.0\\n        [Right, Middle]: 2.0\\n        [Right, Ring]: 1.6\\n        [Right, Pinky]: 1.0\\n\\n  hand_disbalance:\\n    enabled: true\\n    weight: 40.0\\n    normalization:\\n      type: fixed\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  key_costs:\\n    enabled: true\\n    weight: 7.55\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  # bigram metrics\\n  asymmetric_bigrams:\\n    enabled: true\\n    weight: 1.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  finger_repeats:\\n    enabled: true\\n    weight: 780.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      index_finger_factor: 0.9\\n      pinky_finger_factor: 1.2\\n      total_weight_threshold: 20\\n      critical_fraction: 1.00025 # >1 -> do not use\\n      factor: 5.0\\n\\n  finger_repeats_top_bottom:\\n    enabled: true\\n    weight: 1850.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      index_finger_factor: 0.9\\n      pinky_finger_factor: 1.2\\n      total_weight_threshold: 20\\n      critical_fraction: 1.00025 # >1 -> do not use\\n      factor: 5.0\\n\\n  finger_repeats_lateral:\\n    enabled: true\\n    weight: 780.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  line_changes:\\n    enabled: true\\n    weight: 5.5\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      short_fingers: [[\\\"Left\\\", \\\"Pointer\\\"], [\\\"Right\\\", \\\"Pointer\\\"], [\\\"Right\\\", \\\"Pinky\\\"]]  # no left pinky!\\n      long_fingers: [[\\\"Left\\\", \\\"Middle\\\"], [\\\"Left\\\", \\\"Ring\\\"], [\\\"Right\\\", \\\"Middle\\\"], [\\\"Right\\\", \\\"Ring\\\"]]\\n      short_up_to_long_or_long_down_to_short_reduction: 0.25\\n      short_down_to_long_or_long_up_to_short_increase: 0.5\\n      count_row_changes_between_hands: false\\n\\n  manual_bigram_penalty:\\n    enabled: true\\n    weight: 1050.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      matrix_positions:\\n        # all combinations with pinky will be added automatically\\n        #\\n        # symmetric ones will be added automatically\\n        # NOTE: in contrast to ArneBab's layout, we skip one column in rows 0, 1, 2\\n        [[1, 3], [3, 2]]: 1\\n        [[2, 3], [3, 2]]: 0.3\\n        [[2, 4], [3, 3]]: 0.2\\n        [[1, 2], [3, 3]]: 0.2\\n        [[1, 2], [3, 4]]: 0.1\\n\\n        [[3, 11], [1, 7]]: 0.1\\n        [[3, 2], [1, 6]]: 0.1\\n\\n        [[3, 2], [3, 5]]: -0.01\\n        [[3, 8], [3, 11]]: -0.01\\n\\n  movement_pattern:\\n    enabled: true\\n    weight: 50.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      finger_switch_costs:\\n        - { from: [Left, Pinky],   to: [Left, Ring],    cost: 8 }\\n        - { from: [Left, Pinky],   to: [Left, Middle],  cost: 2 }\\n\\n        - { from: [Left, Ring],    to: [Left, Pinky],   cost: 12 }\\n        - { from: [Left, Ring],    to: [Left, Middle],  cost: 6 }\\n        - { from: [Left, Ring],    to: [Left, Pointer], cost: 0.1 }\\n\\n        - { from: [Left, Middle],  to: [Left, Pinky],   cost: 3 }\\n        - { from: [Left, Middle],  to: [Left, Ring],    cost: 9 }\\n        - { from: [Left, Middle],  to: [Left, Pointer], cost: 0.6 }\\n\\n        - { from: [Left, Pointer], to: [Left, Pinky],   cost: 0.1 }\\n        - { from: [Left, Pointer], to: [Left, Ring],    cost: 0.3 }\\n        - { from: [Left, Pointer], to: [Left, Middle],  cost: 0.9 }\\n\\n        - { from: [Right, Pinky],   to: [Right, Ring],    cost: 8 }\\n        - { from: [Right, Pinky],   to: [Right, Middle],  cost: 2 }\\n\\n        - { from: [Right, Ring],    to: [Right, Pinky],   cost: 12 }\\n        - { from: [Right, Ring],    to: [Right, Middle],  cost: 6 }\\n        - { from: [Right, Ring],    to: [Right, Pointer], cost: 0.1 }\\n\\n        - { from: [Right, Middle],  to: [Right, Pinky],   cost: 3 }\\n        - { from: [Right, Middle],  to: [Right, Ring],    cost: 9 }\\n        - { from: [Right, Middle],  to: [Right, Pointer], cost: 0.6 }\\n\\n        - { from: [Right, Pointer], to: [Right, Pinky],   cost: 0.1 }\\n        - { from: [Right, Pointer], to: [Right, Ring],    cost: 0.3 }\\n        - { from: [Right, Pointer], to: [Right, Middle],  cost: 0.9 }\\n\\n  no_handswitch_after_unbalancing_key:\\n    enabled: true\\n    weight: 18.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      unbalancing_after_unbalancing: 4\\n\\n  unbalancing_after_neighboring:\\n    enabled: true\\n    weight: 200.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  # trigram metrics\\n  irregularity:\\n    enabled: true\\n    weight: 8.25\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      null: null\\n\\n  no_handswitch_in_trigram:\\n    enabled: true\\n    weight: 465.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      factor_with_direction_change: 1.0\\n      factor_without_direction_change: 0.0\\n\\n  secondary_bigrams:\\n    enabled: true\\n    weight: 0.2\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      factor_no_handswitch: 0.7\\n      factor_handswitch: 0.8\\n      exclude_containing: [\\\",\\\", \\\".\\\"]\\n\\n  trigram_finger_repeats:\\n    enabled: true\\n    weight: 10000.0\\n    normalization:\\n      type: weight_found\\n      value: 1.0\\n    params:\\n      factor_lateral_movement: 1.2\\n\\nngrams:\\n  unigrams: 1-gramme.arne.no-special.txt\\n  bigrams: 2-gramme.arne.no-special.txt\\n  trigrams: 3-gramme.arne.no-special.txt\\n\\nngram_mapper:\\n  split_modifiers:\\n    enabled: true\\n    same_key_mod_factor: 0.03125\\n\\n  secondary_bigrams_from_trigrams:\\n    enabled: false\\n    factor_no_handswitch: 0.7\\n    factor_handswitch: 0.8\\n    exclude_containing: [\\\",\\\", \\\".\\\"]\\n\\n  increase_common_bigrams:\\n    enabled: false\\n    critical_fraction: 0.001\\n    factor: 2.0\\n    total_weight_threshold: 20.0\\n\");\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/config/evaluation_parameters.yml?");

/***/ }),

/***/ "../../config/standard_keyboard.yml":
/*!**********************************************************************************************************!*\
  !*** /home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/config/standard_keyboard.yml ***!
  \**********************************************************************************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"keyboard:\\n  matrix_positions:\\n    - [[0,0],        [2,0], [3,0], [4,0], [5,0], [6,0],   [7,0], [8,0], [9,0], [10,0], [11,0], [12,0], [13,0], [14,0]]\\n    - [[0,1],          [2,1], [3,1], [4,1], [5,1], [6,1],   [7,1], [8,1], [9,1], [10,1], [11,1], [12,1], [13,1]]\\n    - [[0,2],            [2,2], [3,2], [4,2], [5,2], [6,2],   [7,2], [8,2], [9,2], [10,2], [11,2], [12,2], [13,2], [14,2]]\\n    - [[0,3],    [1,3],    [2,3], [3,3], [4,3], [5,3], [6,3],   [7,3], [8,3], [9,3], [10,3], [11,3], [12,3]]\\n    - [[0,4], [1,4], [2,4],                             [6,4],                        [10,4], [11,4], [12,4], [13,4]]\\n\\n  positions:\\n    - [[   0,0],   [   1,0],   [   2,0],   [   3,0],   [   4,0],   [   5,0],   [   6,0],   [   7,0],   [   8,0],   [   9,0],   [   10,0],   [   11,0],   [   12,0],   [   13,0]]\\n    - [   [0.25,1],   [1.25,1],   [2.25,1],   [3.35,1],   [4.25,1],   [5.25,1],   [6.25,1],   [7.25,1],   [8.25,1],   [9.25,1],   [10.25,1],   [11.25,1],   [12.25,1]]\\n    - [      [ 0.5,2],   [ 1.5,2],   [ 2.5,2],   [ 3.5,2],   [ 4.5,2],   [ 5.5,2],   [ 6.5,2],   [ 7.5,2],   [ 8.5,2],   [ 9.5,2],   [ 10.5,2],   [ 11.5,2],   [ 12.5,2],   [ 13.5,2]]\\n    - [   [0.25,3],   [1.25,3],    [2.25,3],  [3.25,3],   [4.25,3],   [5.25,3],   [6.25,3],   [7.25,3],   [8.25,3],   [9.25,3],   [10.25,3],   [11.25,3],   [12.25,3]]\\n    - [      [ 0.5,4],         [   2,4],   [   3,4],                                       [   7,4],                                        [   11,4],   [ 12.5,4],   [   13,4],    [   14,4]]\\n\\n  hands:\\n    - [Left,       Left, Left, Left, Left, Left,   Right, Right, Right, Right, Right, Right, Right, Right]\\n    - [Left,         Left, Left, Left, Left, Left,   Right, Right, Right, Right, Right, Right, Right]\\n    - [Left,           Left, Left, Left, Left, Left,   Right, Right, Right, Right, Right, Right, Right, Right]\\n    - [Left,    Left,    Left, Left, Left, Left, Left,   Right, Right, Right, Right, Right, Right]\\n    - [Left, Left, Left,                     Right,                      Right, Right, Right, Right]\\n\\n  fingers:\\n    - [Pinky,        Pinky, Ring, Middle, Pointer, Pointer,   Pointer, Pointer, Middle, Ring, Pinky, Pinky, Pinky, Pinky]\\n    - [Pinky,          Pinky, Ring, Middle, Pointer, Pointer,   Pointer, Pointer, Middle, Ring, Pinky, Pinky, Pinky]\\n    - [Pinky,            Pinky, Ring, Middle, Pointer, Pointer,   Pointer, Pointer, Middle, Ring, Pinky, Pinky, Pinky, Pinky]\\n    - [Pinky,    Pinky,    Pinky, Ring, Middle, Pointer, Pointer,   Pointer, Pointer, Middle, Ring, Pinky, Pinky]\\n    - [Pinky, Ring, Thumb,                          Thumb,                      Thumb, Middle, Ring, Pinky]\\n\\n  key_costs:\\n    - [80,     70, 60, 50, 50, 60,   60, 50, 50, 50, 50, 60, 70, 80]\\n    - [24,      16, 10,  5, 12, 17,   20, 13,  5,  9, 11, 20, 36]\\n    - [ 9,         5,  3,  3,  3,  6,    6,  3,  3,  3,  5,  9, 30,  6]\\n    - [20,   16,    19, 24, 20,  9, 30,   10,  8, 22, 22, 17, 19]\\n    - [ 0,  0,  0,              3,               7,  0,  0,  0]\\n\\n  unbalancing_positions:\\n    - [  2,         2,   2,   0,   0,   0,     0,   0,   0,   0,   2,   2,   2,   2]\\n    - [  2,           1,   0,   0, 0.1,   2,     2, 0.1,   0,   0,   1,   2, 2.5]\\n    - [  1,             0,   0,   0,   0,   1,     1,   0,   0,   0,   0,   1,   2,   2]\\n    - [  2,      0,      0, 0.5, 0.5,   0,   2,    0.5,   0, 0.5, 0.5,   0,   2]\\n    - [  3,   0,   0,                       0,                        0,   0,   0,   3]\\n\\n  symmetries:\\n    - [  1,         2,   3,   4,   5,   6,     6,   5,   4,   3,   2,   1,   7,   8]\\n    - [  9,          10,  11,  12,  13,  14,    14,  13,  12,  11,  10,   9,  15]\\n    - [ 16,            17,  18,  19,  20,  21,    21,  20,  19,  18,  17,  16,  22,  23]\\n    - [ 24,     25,     26,  27,  28,  29,  30,     30,  29,  28,  27,  26,  24]\\n    - [ 31,  32,  33,                      34,                       35,  36,  37,  38]\\n\\n\\n  plot_template: |\\n    ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬──────┐\\n    │ {{0}} │ {{1}} │ {{2}} │ {{3}} │ {{4}} │ {{5}} │ {{6}} │ {{7}} │ {{8}} │ {{9}} │ {{10}} │ {{11}} │ {{12}} │ {{13}}    │\\n    ├───┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬────┤\\n    │   {{14}} │ {{15}} │ {{16}} │ {{17}} │ {{18}} │ {{19}} │ {{20}} │ {{21}} │ {{22}} │ {{23}} │ {{24}} │ {{25}} │ {{26}} │ Ret│\\n    ├─────┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┐   │\\n    │    {{27}} │ {{28}} │ {{29}} │ {{30}} │ {{31}} │ {{32}} │ {{33}} │ {{34}} │ {{35}} │ {{36}} │ {{37}} │ {{38}} │ {{39}} │ {{40}} │\\n    ├────┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴───┴───┤\\n    │  {{41}} │ {{42}} │ {{43}} │ {{44}} │ {{45}} │ {{46}} │ {{47}} │ {{48}} │ {{49}} │ {{50}} │ {{51}} │ {{52}} │    {{53}}    │\\n    ├────┼───┴┬──┴─┬─┴───┴───┴───┴───┴───┴─┬─┴──┬┴───┼────┬────┤\\n    │  {{54}} │ {{55}}  │ {{56}}  │           {{57}}           │  {{58}} │  {{59}} │  {{60}} │  {{61}} │\\n    └────┴────┴────┴───────────────────────┴────┴────┴────┴────┘\\n\\n  plot_template_short: |\\n   {{0}}{{1}}{{2}}{{3}}{{4}}{{5}} {{6}}{{7}}{{8}}{{9}}{{10}}{{11}}\\n   {{12}}{{13}}{{14}}{{15}}{{16}} {{17}}{{18}}{{19}}{{20}}{{21}}⇘\\n   {{22}}{{23}}{{24}}{{25}}{{26}} {{27}}{{28}}{{29}}{{30}}{{31}}\\n\\nbase_layout:\\n  keys:\\n    # Zahlenreihe [0]\\n    - - [\\\"^\\\", \\\"ˇ\\\", \\\"↻\\\", \\\"˙\\\", \\\"˞\\\", \\\"̣\\\"]\\n      - [\\\"1\\\", \\\"°\\\", \\\"¹\\\", \\\"ª\\\", \\\"₁\\\", \\\"¬\\\"]\\n      - [\\\"2\\\", \\\"§\\\", \\\"²\\\", \\\"º\\\", \\\"₂\\\", \\\"∨\\\"]\\n      - [\\\"3\\\", \\\"ℓ\\\", \\\"³\\\", \\\"№\\\", \\\"₃\\\", \\\"∧\\\"]\\n      - [\\\"4\\\", \\\"»\\\", \\\"›\\\", \\\"\\\", \\\"♀\\\", \\\"⊥\\\"]\\n      - [\\\"5\\\", \\\"«\\\", \\\"‹\\\", \\\"·\\\", \\\"♂\\\", \\\"∡\\\"]\\n      - [\\\"6\\\", \\\"$\\\", \\\"¢\\\", \\\"£\\\", \\\"⚥\\\", \\\"∥\\\"]\\n      - [\\\"7\\\", \\\"€\\\", \\\"¥\\\", \\\"¤\\\", \\\"ϰ\\\", \\\"→\\\"]\\n      - [\\\"8\\\", \\\"„\\\", \\\"‚\\\", \\\"⇥\\\", \\\"⟨\\\", \\\"∞\\\"]\\n      - [\\\"9\\\", \\\"“\\\", \\\"‘\\\", \\\"/\\\", \\\"⟩\\\", \\\"∝\\\"]\\n      - [\\\"0\\\", \\\"”\\\", \\\"’\\\", \\\"*\\\", \\\"₀\\\", \\\"∅\\\"]\\n      - [\\\"-\\\", \\\"—\\\", \\\"-\\\", \\\"‑\\\", \\\"­\\\", \\\"\\\"]\\n      - [\\\"`\\\", \\\"¸\\\", \\\"°\\\", \\\"¨\\\", \\\"\\\", \\\"¯\\\"]\\n      - [\\\"←\\\"]\\n    # Reihe 1\\n    - - [\\\"⇥\\\"]\\n      - [\\\"x\\\", \\\"X\\\", \\\"…\\\", \\\"⇞\\\", \\\"ξ\\\", \\\"Ξ\\\"]\\n      - [\\\"v\\\", \\\"V\\\", \\\"_\\\", \\\"⌫\\\", \\\"\\\", \\\"√\\\"]\\n      - [\\\"l\\\", \\\"L\\\", \\\"[\\\", \\\"⇡\\\", \\\"λ\\\", \\\"Λ\\\"]\\n      - [\\\"c\\\", \\\"C\\\", \\\"]\\\", \\\"⌦\\\", \\\"χ\\\", \\\"ℂ\\\"]\\n      - [\\\"w\\\", \\\"W\\\", \\\"^\\\", \\\"⇟\\\", \\\"ω\\\", \\\"Ω\\\"]\\n      - [\\\"k\\\", \\\"K\\\", \\\"!\\\", \\\"¡\\\", \\\"κ\\\", \\\"×\\\"]\\n      - [\\\"h\\\", \\\"H\\\", \\\"<\\\", \\\"7\\\", \\\"ψ\\\", \\\"Ψ\\\"]\\n      - [\\\"g\\\", \\\"G\\\", \\\">\\\", \\\"8\\\", \\\"γ\\\", \\\"Γ\\\"]\\n      - [\\\"f\\\", \\\"F\\\", \\\"=\\\", \\\"9\\\", \\\"φ\\\", \\\"Φ\\\"]\\n      - [\\\"q\\\", \\\"Q\\\", \\\"&\\\", \\\"+\\\", \\\"ϕ\\\", \\\"ℚ\\\"]\\n      - [\\\"y\\\", \\\"Y\\\", \\\"@\\\", \\\".\\\", \\\"υ\\\", \\\"∇\\\"]\\n      - [\\\"ß\\\", \\\"ẞ\\\", \\\"ſ\\\", \\\"−\\\", \\\"ς\\\", \\\"∘\\\"]\\n    # Reihe 2\\n    - - [\\\"⇩\\\"]\\n      - [\\\"u\\\", \\\"U\\\", \\\"\\\\\\\\\\\", \\\"⇱\\\", \\\"\\\", \\\"⊂\\\"]\\n      - [\\\"i\\\", \\\"I\\\", \\\"/\\\", \\\"⇠\\\", \\\"ι\\\", \\\"∫\\\"]\\n      - [\\\"a\\\", \\\"A\\\", \\\"{\\\",  \\\"⇣\\\", \\\"α\\\", \\\"∀\\\"]\\n      - [\\\"e\\\", \\\"E\\\", \\\"}\\\", \\\"⇢\\\", \\\"ε\\\", \\\"∃\\\"]\\n      - [\\\"o\\\", \\\"O\\\", \\\"*\\\", \\\"⇲\\\", \\\"ο\\\", \\\"∈\\\"]\\n      - [\\\"s\\\", \\\"S\\\", \\\"?\\\", \\\"¿\\\", \\\"σ\\\", \\\"Σ\\\"]\\n      - [\\\"n\\\", \\\"N\\\", \\\"(\\\", \\\"4\\\", \\\"ν\\\", \\\"ℕ\\\"]\\n      - [\\\"r\\\", \\\"R\\\", \\\")\\\", \\\"5\\\", \\\"ρ\\\", \\\"ℝ\\\"]\\n      - [\\\"t\\\", \\\"T\\\", \\\"-\\\", \\\"6\\\", \\\"τ\\\", \\\"∂\\\"]\\n      - [\\\"d\\\", \\\"D\\\", \\\":\\\", \\\",\\\", \\\"δ\\\", \\\"Δ\\\"]\\n      - [\\\"⇘\\\"]\\n      - [\\\"´\\\", \\\"~\\\", \\\"/\\\", \\\"˝\\\", \\\"\\\", \\\"˘\\\"]\\n      - [\\\"\\\\n\\\"]\\n    # Reihe 3\\n    - - [\\\"⇧\\\"]\\n      - [\\\"⇚\\\"]\\n      - [\\\"ü\\\", \\\"Ü\\\", \\\"#\\\", \\\"\\u001b\\\", \\\"\\\", \\\"∪\\\"]\\n      - [\\\"ö\\\", \\\"Ö\\\", \\\"$\\\", \\\"\\\\t\\\", \\\"ϵ\\\", \\\"∩\\\"]\\n      - [\\\"ä\\\", \\\"Ä\\\", \\\"|\\\", \\\"⎀\\\", \\\"η\\\", \\\"ℵ\\\"]\\n      - [\\\"p\\\", \\\"P\\\", \\\"~\\\", \\\"\\\\n\\\", \\\"π\\\", \\\"Π\\\"]\\n      - [\\\"z\\\", \\\"Z\\\", \\\"`\\\", \\\"↶\\\", \\\"ζ\\\", \\\"ℤ\\\"]\\n      - [\\\"b\\\", \\\"B\\\", \\\"+\\\", \\\":\\\", \\\"β\\\", \\\"⇐\\\"]\\n      - [\\\"m\\\", \\\"M\\\", \\\"%\\\", \\\"1\\\", \\\"μ\\\", \\\"⇔\\\"]\\n      - [\\\",\\\", \\\"–\\\", '\\\"', \\\"2\\\", \\\"ϱ\\\", \\\"⇒\\\"]\\n      - [\\\".\\\", \\\"•\\\", \\\"'\\\", \\\"3\\\", \\\"ϑ\\\", \\\"↦\\\"]\\n      - [\\\"j\\\", \\\"J\\\", \\\";\\\", \\\";\\\", \\\"θ\\\", \\\"Θ\\\"]\\n      - [\\\"⇗\\\"]\\n    # Reihe 4 mit Leertaste\\n    - - [\\\"♕\\\"]\\n      - [\\\"\\\"]\\n      - [\\\"♔\\\"]\\n      - [\\\" \\\", \\\" \\\", \\\" \\\", \\\"0\\\", \\\" \\\", \\\" \\\"]\\n      - [\\\"⇙\\\"]\\n      - [\\\"\\\"]\\n      - [\\\"\\\"]\\n      - [\\\"♛\\\"]\\n\\n  fixed_keys:\\n    - [ true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true]\\n    - [ true, false, false, false, false, false, false, false, false, false, false, false, false]\\n    - [ true, false, false, false, false, false, false, false, false, false, false,  true,  true,  true]\\n    - [ true,  true, false, false, false, false, false, false, false, false, false, false,  true]\\n    - [ true,  true,  true,                       true,                       true,  true,  true,  true]\\n\\n  fixed_layers: [2, 3]\\n\\n  modifiers:\\n    - Left: [\\\"⇧\\\"]\\n      Right: [\\\"⇗\\\"]\\n    - Left: [\\\"⇩\\\"]\\n      Right: [\\\"⇘\\\"]\\n    - Left: [\\\"⇚\\\"]\\n      Right: [\\\"⇙\\\"]\\n    - Left: [\\\"⇧\\\", \\\"⇚\\\"]\\n      Right: [\\\"⇗\\\", \\\"⇙\\\"]\\n    - Left: [\\\"⇩\\\", \\\"⇚\\\"]\\n      Right: [\\\"⇘\\\", \\\"⇙\\\"]\\n\\n  layer_costs: [0, 20, 9, 16, 29, 25]\\n\");\n\n//# sourceURL=webpack:////home/runner/work/keyboard_layout_optimizer/keyboard_layout_optimizer/config/standard_keyboard.yml?");

/***/ }),

/***/ "./app.js":
/*!****************!*\
  !*** ./app.js ***!
  \****************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _config_standard_keyboard_yml__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../config/standard_keyboard.yml */ \"../../config/standard_keyboard.yml\");\n/* harmony import */ var _config_evaluation_parameters_yml__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../config/evaluation_parameters.yml */ \"../../config/evaluation_parameters.yml\");\n\n\n\nconst NKEYS = 32\n\nVue.component('evaluator-app', {\n    template: `\n<b-container>\n\n  <h1>Keyboard Layout Evaluator</h1>\n  <hr>\n\n  <b-row>\n    <b-col xl=\"6\">\n      <b-form inline @submit.stop.prevent @submit=\"evaluate\">\n        <b-form-input v-model=\"layoutRaw\" placeholder=\"Layout\" class=\"mb-2 mr-sm-2 mb-sm-0\" ></b-form-input>\n        <b-button :disabled=\"loading\" @click=\"evaluate\" variant=\"primary\">\n          <div v-if=\"loading\"><b-spinner small></b-spinner> Loading</div>\n          <div v-else>Evaluate</div>\n        </b-button>\n      </b-form>\n      <layout-plot :layout-string=\"layout\" :wasm=\"wasm\"></layout-plot>\n      <layout-details v-if=\"details !== null\" title=\"Details\" :layout-details=\"details\"></layout-details>\n    </b-col>\n\n    <b-col v-if=\"details !== null\" xl=\"6\">\n      <b-form inline>\n        <b-form-checkbox v-model=\"relative\"inline>relative barplot</b-form-checkbox>\n        <b-form-checkbox v-if=\"!relative\" v-model=\"logscale\" inline>logarithmic scale</b-form-checkbox>\n      </b-form>\n      <layout-barplot :layout-details=\"detailsArray\" :relative=\"relative\" :logscale=\"logscale && !relative\" :styles=\"chartStyles\"></layout-barplot>\n    </b-col>\n  </b-row>\n\n</b-container>\n`,\n    props: {\n        relative: { type: Boolean, default: false },\n        logscale: { type: Boolean, default: false },\n    },\n    data () {\n        return {\n            details: null,\n            layoutRaw: null,\n            layoutEvaluator: null,\n            frequenciesNgramProvider: null,\n            wasm: null,\n            loading: true,\n        }\n    },\n    computed: {\n        detailsArray () {\n            if (this.details === null) return []\n            return [this.details]\n        },\n        chartStyles () {\n            return {\n                height: \"600px\",\n                position: \"relative\"\n            }\n        },\n        layout () {\n            let layoutString = (this.layoutRaw || \"\").replace(\" \", \"\")\n            return layoutString\n        },\n    },\n    created () {\n        let wasm_import = __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! evolve-keyboard-layout-wasm */ \"../pkg/evolve_keyboard_layout_wasm.js\"))\n        let unigram_import = __webpack_require__.e(/*! import() */ 2).then(__webpack_require__.bind(null, /*! ../../1-gramme.arne.no-special.txt */ \"../../1-gramme.arne.no-special.txt\"))\n        let bigram_import = __webpack_require__.e(/*! import() */ 3).then(__webpack_require__.bind(null, /*! ../../2-gramme.arne.no-special.txt */ \"../../2-gramme.arne.no-special.txt\"))\n        let trigram_import = __webpack_require__.e(/*! import() */ 4).then(__webpack_require__.bind(null, /*! ../../3-gramme.arne.no-special.txt */ \"../../3-gramme.arne.no-special.txt\"))\n\n        wasm_import.then((wasm) => {\n            this.wasm = wasm\n        })\n\n        Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])\n        .then((imports) => {\n            let wasm = imports[0]\n            let unigrams = imports[1].default\n            let bigrams = imports[2].default\n            let trigrams = imports[3].default\n\n            this.frequenciesNgramProvider = this.wasm.NgramProvider.with_frequencies(\n                _config_evaluation_parameters_yml__WEBPACK_IMPORTED_MODULE_1__[\"default\"],\n                unigrams,\n                bigrams,\n                trigrams\n            )\n\n            this.layoutEvaluator = this.wasm.LayoutEvaluator.new(\n                _config_standard_keyboard_yml__WEBPACK_IMPORTED_MODULE_0__[\"default\"],\n                _config_evaluation_parameters_yml__WEBPACK_IMPORTED_MODULE_1__[\"default\"],\n                this.frequenciesNgramProvider\n            )\n\n            this.loading = false\n        })\n    },\n    methods: {\n        evaluate () {\n            if (this.layout.length !== NKEYS) {\n                this.$bvToast.toast(\"Keyboard layout must have 32 (non-whitespace) symbols\", {variant: \"danger\"})\n                return\n            }\n            try {\n                let details = this.layoutEvaluator.evaluate(this.layout)\n                details.layout = this.layout\n                this.details = details\n            } catch(err) {\n                this.$bvToast.toast(`Could not generate a valid layout: ${err}`, {variant: \"danger\"})\n                return\n            }\n        }\n    }\n})\n\n\nVue.component('layout-plot', {\n    template: `\n    <pre><code>\n{{plotString}}\n    </code></pre>\n`,\n    props: {\n        layoutString: { type: String, default: \"\" },\n        defaultSymbol: { type: String, default: \".\" },\n        wasm: { type: Object, default: null },\n    },\n    data () {\n        return {\n            plotString: null,\n            layoutPlotter: null,\n        }\n    },\n    watch: {\n        layoutString () {\n            this.plot()\n        },\n        wasm () {\n            if (this.wasm === null) return\n            this.layoutPlotter = this.wasm.LayoutPlotter.new(_config_standard_keyboard_yml__WEBPACK_IMPORTED_MODULE_0__[\"default\"])\n            this.plot()\n        },\n    },\n    methods: {\n        plot () {\n            if (this.layoutPlotter === null) return \"\"\n\n            const nMissing = NKEYS - this.layoutString.length\n            if (nMissing < 0) {\n                this.$bvToast.toast(`Too many symbols given (${this.layoutString.length} > ${NKEYS})`, {variant: \"danger\"})\n                return\n            }\n            let layout = this.layoutString + Array(nMissing + 1).join(this.defaultSymbol)\n            try {\n                this.plotString = this.layoutPlotter.plot(layout, 0)\n            } catch (err) {\n                this.$bvToast.toast(`Could not plot layout: ${err}`, {variant: \"danger\"})\n                return\n            }\n        },\n    },\n})\n\nvar app = new Vue({\n    el: '#app',\n})\n\n\n//# sourceURL=webpack:///./app.js?");

/***/ })

}]);