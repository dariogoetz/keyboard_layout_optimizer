# Keyboard Layout Optimizer

Keyboard layout optimizer written in rust. The optimizer is based on the "evolve-keyboard-layout" [scripts by ArneBab](https://hg.sr.ht/~arnebab/evolve-keyboard-layout).
It was historically developed with layouts of the ["Neo"-family](https://neo-layout.org/) in mind, but can be applied to arbitrary layouts. It supports the use of multiple layers per key (that are activated by holding corresponding modifiers).

At the heart of the optimization lies a layout evaluation that involves multiple criteria/metrics on the frequencies of unigrams, bigrams, and trigrams. And with a little bit of Rust-knowledge, new metrics [can easily be added](#adding-new-metrics).

For the optimization, individual layers can be excluded from permutations, e.g. in the default configuration, permutations are performed in the base layer and layers 2, 5, and 6 whereas layers 3 and 4 remain unchanged (in the spirit of "Neo"-family layouts).

## Webapp - Evaluation and Optimization
There is a webapp providing (a significant subset of) the evaluation and optimization functionalities at https://dariogoetz.github.io/keyboard_layout_optimizer.

The corresponding webapp implementation is located in the `webui/layout_evaluation_wasm` crate.

## Webapp - Result Exploration

Published results can be explored and compared at https://keyboard-layout-optimizer.fly.dev (previously https://keyboard-layout-optimizer.herokuapp.com).

The corresponding webserver's implementation is located in the `webui/layouts_webservice` crate.

## Features
- evaluation based on unigrams, bigrams, and trigrams
- support for higher layer characters (e.g. uppercase letters or symbols) by expanding ngrams with modifier keys
- arbitrary positioning of modifier keys (e.g. for home-row-mods)
- flexible configuration options for metrics and keyboards (e.g. configs for ergo-boards)
- fast evaluation (~100ms per layout including trigram metrics even for large corpora &gt; 100 MB)
- layout optimization using [various algorithms](#optimization-algorithms)

## Metrics
- **key costs** - How do the letter frequencies relate to the "cost" associated to the keys?
- **finger repeats** - How often are fingers in action consecutively?
- **movement pattern** - How comfortable is it to type individual bigrams? Which finger follows which? How many rows? Upwards/downwards?
- **finger balance** - Is each finger suitably loaded? Pinkies less than index fingers?
- **hand disbalance** - Are left and right hands similarly loaded?
- **no handswitch after unbalancing key** - How often does no handswitch occur after a hand needed to move away from the home row?
- **irregularity** - How often are the first and the second bigram in a trigram "bad" (wrt. to all bigram metrics)?
- **secondary bigrams** - How compatible are first and third keys of a trigram?
- **no handswitch in trigram** - How often does no handswitch happen within a trigram (and have a direction change in between)?
- **badly positioned shortcut keys** - How many shorcut keys are not easily reachable with the left hand?
- **similar letters** - (learnability) Which keys are similar (in some sense), but lie in unsimilar locations (e.g. "a" - "ä" or "b" - "p")?
- **similar letter-groups** - (learnability) Which groups of keys are similar (in some sense), but lie in non-consistent locations (e.g. "aou" - "äüö")?<br>Used to be called "asymmetric keys".
- **KLAnext metrics (distance, same-hand, same-finger)** - A re-implementation of the metrics used by the [KLAnext layout evaluator](https://klanext.keyboard-design.com)
- **word-based metrics used in the [Internet Letter Layout DB](https://keyboard-design.com/internet-letter-layout-db.html)** - How many of the most used 30,000 words can be written without a finger repeat / on the home-row?

## Installation
1. Clone the repository
    ``` sh
    git clone https://github.com/dariogoetz/keyboard_layout_optimizer.git
    ```
1. Build the binaries (add `CC=gcc` in the beginning if `cc` is not installed, but `gcc` is)
    ``` sh
    cargo build --release
    ```
    The binaries are then located under `target/release`.

    Alternatively, run the binaries directly using
    ``` sh
    cargo run --release --bin [...]
    ```
1. Generate documentation with
   ``` sh
   cargo doc
   ```

## Usage
### Specifying Layouts
Some binaries expect layouts as commandline arguments. These layouts are represented as strings specifying the keys of the layout from left to right, top to bottom, i.e. it starts on the top left of the keyboard and lists each letter of the base layer going to the right in the same row. After that the letters of the next row follow, again from left to right.

Whitespace is allowed and will be ignored.

Only those keys shall be specified that are not marked as "fixed" in the layout configuration file "config/keyboard/standard.yml" (usually 32 keys).

There are two options how the layout string provided on the commandline is interpreted:
#### Default Behavior
Only the keys of the "base layer" are specified in the provided layout string (corresponding to the first symbols of the lists defined in the config under `base_layout`).
The the base layer symbols together with all upper layer symbols defined in the `base_layout` move to the specified location (except those layers defined in `fixed_layers`).

Using this option, optimizations always keep the symbols defined in the `base_layout` together (apart from the `fixed_layers` that do not permute at all).

#### Grouped Layers (used if the commandline-argument `--grouped-layout-generator` is active)
The number of symbols provided in the given layout string can be a multiple of the non-fixed keys, say `N`. In that case, the first `N` symbols represent the first layer of the layout.
The config parameter `grouped_layers` determines the number of symbols in the `base_layout` that move together with the symbol in the given layout string.
For instance, `grouped_layers: 1` means that only the given symbol moves to the specified location. `grouped_layers: 2` would move the given symbol together with the next symbol in the same list (maybe its uppercase variant).

The second `N` symbols are then placed in the next layer of the layout (layer `grouped_layers + 1`).

This option allows optimizing the location of symbols across multiple layers independently.

### Layout Plot Binary
The `plot` binary expects a layout representation as commandline argument.

Example (Bone layout):
``` sh
RUST_LOG=INFO ./target/release/plot "jduax phlmwqß ctieo bnrsg fvüäö yz,.k"
```

As an optional parameter `--layout-config`, a different layout configuration file can be specified.

### Layout Evaluation Binary
The `evaluate` binary expects a layout representation as commandline argument.

Example (Bone layout):
``` sh
RUST_LOG=INFO ./target/release/evaluate "jduax phlmwqß ctieo bnrsg fvüäö yz,.k"
```

There are various optional parameters that can be explored using the `-h` option, e.g. provide a text or file to be used as corpus.

#### Configuration
Many aspects of the evaluation can be configured in the yaml files `config/keyboard/standard.yml` and `config/evaluation/default.yml`.

##### `config/keyboard/standard.yml`
This file contains "physical" properties of the keyboard and information about the Neo layout that serves as an underlying base for the variants to evaluate. It covers for the keyboard:
- key positions
- key to hand mapping
- key to finger mapping
- key costs (used for evaluation)
- keys that are "unbalancing" the hand's position when hit
- symmetries
- plot templates

And for the Neo base layout:
- the symbols that can be generated in each layer over each key
- keys that can not be permutated
- modifiers to be used to access each layer

Alternatively to `standard.yml`, there are variants for split/ortho keyboards
(`ortho.yml` - a generic ortholinear split keyboard, `moonlander.yml` - the ZSA moonlander
keyboard, `crkbd.yml` - the corne aka. crkbd split keyboard) and variants based on US and UK QWERTY
base layouts instead of neo (`standard_qwerty_uk.yml` and `standard_qwerty_us.yml`).

##### `config/evaluation/default.yml`
This file contains configuration parameters for all available evaluation metrics, filenames of prepared ngram data to use, and parameters specifying the behavior of post-processing the ngram data for a given layout.

### Layout Optimization Binary
The available optimize-binaries include `optimize_genetic.rs` and `optimize_sa.rs`.
If run without any commandline parameters, they start with a random layout or a collection of random layouts and optimize from there. With commandline options, a "starting layout" can be specified or a list of keys that shall not be permutated (if no starting layout is given, fixed keys relate to the [Neo2](https://neo-layout.org/) layout).
Optional commandline parameters can be explored with the `-h` option.

Example for a never ending search (appends solutions to a file `found_solutions.txt` and publishes them to https://keyboard-layout-optimizer.fly.dev):

``` sh
RUST_LOG=INFO ./target/release/optimize_genetic --run-forever --append-solutions-to "found_solutions.txt" --publish-as "<your name>"
```

#### Optimization Algorithms
Choosing an algorithm:
- [Simulated Annealing](#simulated-annealing-optimize_sars) produces the best layouts from scratch.
- To optimize a preexisting layout while keeping it similar to the original, [Genetic](#genetic-algorithm-optimize_geneticrs) optimization is best suited.

##### Genetic Algorithm (`optimize_genetic.rs`)
Example (starting from Bone layout, fixing "," and "."):
``` sh
RUST_LOG=INFO ./target/release/optimize_genetic -s "jduax phlmwqß ctieo bnrsg fvüäö yz,.k" -f ",."
```

##### Simulated Annealing (`optimize_sa.rs`)
An explanation of Simulated Annealing can be found [here](https://en.wikipedia.org/wiki/Simulated_annealing).

Example (starting from Bone layout, fixing "," and "."):
``` sh
RUST_LOG=INFO ./target/release/optimize_sa -s "jduax phlmwqß ctieo bnrsg fvüäö yz,.k" -f ",."
```
In contrast to other binaries, using this algorithm you can optimize multiple starting-layouts simultaneously. Example of an optimization (starting from Bone, Neo, and KOY):
``` sh
RUST_LOG=INFO ./target/release/optimize_sa -s "jduaxphlmwqßctieobnrsgfvüäöyz,.k" -s "xvlcwkhgfqyßuiaeosnrtdüöäpzbm,.j" -s "k.o,yvgclfzßhaeiudtrnsxqäüöbpwmj"
```

#### Configuration
The parameters of the corresponding optimization process can be configured in the files:
* `genetic.yml`
* `sa.yml`

They can be found inside the config-directory (`config/optimization/`).

### Environment Variables
The following environment variables can be set to influence the runtime behavior of the evaluation and
optimization binaries.

- `RAYON_NUM_THREADS`: Number of threads to use for parallel evaluation. Defaults to the number of
  CPU cores.
- `SHOW_WORST`: Determine those ngrams with highest share of the metrics' total costs. Setting this to
  `false` can lead to around 30% increase in evaluation performance, but will leave some parts of
  the result output empty (the actual evaluation scores remain identical). Defaults to `true` for
  `evaluate` and to `false` for the optimization binaries.
- `N_WORST`: The number of ngrams with highest share of the metrics' total costs to show in the
  evaluation output. Higher values increase evaluation time. Defaults to `3`.

## Structure
The project includes several binaries within the `keyboard_layout_optimizer` crate:
1. `plot` - Plots all layers (neo-layouts have six layers) of a specified layout
1. `evaluate` - Evaluates a specified layout and prints a summary of the various metrics to stdout
1. `optimize_genetic` - Starts an optimization heuristic to find a good layout (genetic algorithm)
1. `optimize_sa` - Starts an optimization heuristic to find a good layout (simulated annealing algorithm)
1. `random_evaluate` - Evaluates a series of randomly generated layouts (mostly used for benchmarking)
1. `ngrams` - Generates ngram-frequency files (used as standard input to the evaluation) from a
   given text file
1. `ngram_merge` - Merges multiple ngram-frequency files with given weights into a new one

The binaries rely on three library crates providing relevant data structures and algorithms:
1. `keyboard_layout` - Provides a representation of keys, keyboards, and layouts and a layout generator that generates layout objects from given strings.
1. `layout_evaluation` - Provides functionalities for reading, generating, and processing ngram data and datastructures and traits for evaluating several metrics.
1. `layout_optimization` - Provides optimization functionality. Based on the evaluator in `layout_evaluation`.

Additionally, two web-UIs can be generated in the `webui` directory:
1. `evaluation_wasm` - A static page providing layout evaluation and optimization functionality based on WASM.
1. `layouts_webservice` - A webserver managing a database for collecting layouts and serving a frontend for exploring and comparing them.


## Adding New Metrics
Adding your own metrics is quite simple if you have some programming knowledge. The code for all metrics resides in `layout_evaluation/src/metrics/{layout|unigram|bigram|trigram}_metrics`. Before starting to code, you should determine, whether your new metric assigns cost values to a unigram (single keypress), bigram (two consecutive keypresses), trigram (three consecutive keypresses), or does not rely on any frequency data and only considers the layout itself.

Depending on the choice of metric, replace `{layout|unigram|bigram|trigram}` with the one relevant value in the following.

1. Add a new file `my_metric_name.rs` in the corresponding directory. It will contain the evaluation logic of the metric.

1. The new file should contain
    - a `Parameters` struct with the parameters that will be configurable in the YAML config and
    - a `MyMetricName` struct holding data required for the evaluation (usually only the parameters from the `Parameters` struct)

 1. In order to make the `MyMetricName` struct into a uni-, bi-, or trigram metric, it needs to implement the `{Unigram|Bigram|Trigram}Metric` trait. For that, it is required to implement two functions:
    - the `name` function that simply returns the metric's name, e.g. `"My Metric"` and
    - the `individual_cost` function that assigns a cost value to a single n-gram.

    Optionally, you can also implement the `total_cost` function that receives a slice of n-grams, but in most cases the default implementation suffices (it calls the `individual_cost` function for each n-gram).

    If your metric is a layout metric, there is no `individual_cost` function (as there are no individual n-grams to consider). In that case, you need to implement the `total_cost` function.

1. The `MyMetricName` struct should also have a `new` function for generating a new instance. It receives an instance of `Parameters`.

1. The main parameters of the `individual_cost` function are one/two/three `LayerKey` elements for the keys that belong to the individual uni-/bi-/trigram and the weight of the bigram (how often it occurs in the corpus).

    A `LayerKey` contains all relevant data about the symbol and associated key, such as the position on the keyboard, which hand and finger are used to hit the key, or the associated cost. It also contains the number of the layer in which the symbol lays on the key. If "splitting modifiers" is enabled, this is always `0`, however, as the higher layers have been resolved by adding appropriate modifier keypresses to the n-grams.

    The `individual_cost` function returns a "weighted cost" incorporating the `weight` parameter if necessary, e.g. `Some(weight * cost)`.

1. Make the new module accessible by adding a new line
    ``` rust
    pub mod my_metric_name;
    ```
    at the top of the file `layout_evaluation/src/metrics/{layout|unigram|bigram|trigram}_metrics.rs`.

1. Register the new metric to be used in the `Evaluator` in `layout_evaluation/src/evaluation.rs`. For that,
    - add the line
        ``` rust
        pub my_metric_name: Option<WeightedParams<{layout_|unigram|bigram|trigram}_metrics::my_metric_name::Parameters>>,
        ```
        in the `MetricParameters` struct in order to make the YAML configuration available to your metric
    - generate an instance of your metric by adding the following to the `default_metrics` function of the `Evaluator`:
        ```rust
            add_metric!({layout|unigram|bigram|trigram}_metric, my_metric_name, MyMetricName);
        ```

1. Add a section for the new metric to the config `config/evaluation/default.yml`:
    ```yaml
    my_metric_name:
      enabled: true
      weight: 1.0
      normalization:
        type: weight_found
        value: 1.0
      params:
        null: null
    ```
