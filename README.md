# Keyboard Layout Optimizer

Neo variant layout optimizer written in rust. The optimizer is based on the "evolve-keyboard-layout" [scripts by ArneBab](https://hg.sr.ht/~arnebab/evolve-keyboard-layout).
It supports layouts of the ["Neo"-family](https://neo-layout.org/), i.e. permutations of the base layer, where layers 2, 5, and 6 follow the permutation and layers 3 and 4 remain unchanged.

At the heart of the optimization lies a layout evaluation that involves multiple criteria on the frequencies of unigrams, bigrams, and trigrams.

## Webapp - Evaluation and Optimization
There is a webapp providing (a significant subset of) the evaluation and optimization functionalities at https://dariogoetz.github.io/keyboard_layout_optimizer.

The corresponding webapp implementation is located in the `webui/layout_evaluation_wasm` crate.

## Webapp - Result Exploration

Published results can be explored and compared at https://keyboard-layout-optimizer.herokuapp.com.

The corresponding webserver's implementation is located in the `webui/layouts_webservice` crate.

## Features
- evaluation of keyboard layouts of the ["Neo" family](https://neo-layout.org/)
- evaluation based on prepared unigrams, bigrams, and trigrams or a text
- fast evaluation (~100ms per layout for standard corpus)
- layout optimization using [various algorithms](#optimization-algorithms)
- accounting for higher layer characters (e.g. uppercase letters) by expanding ngrams with modifier keys

## Metrics
- **badly positioned shortcut keys** - How many shorcut keys are not easily reachable with the left hand?
- **similar letters** - Which keys are similar (in some sense), but lie in unsimilar locations (e.g. "a" - "ä" or "b" - "p")?
- **similar letter-groups** - Which groups of keys are similar (in some sense), but lie in non-consistent locations (e.g. "aou" - "äüö")?<br>Used to be called "asymmetric keys".
- **key costs** - How do the letter frequencies relate to the "cost" associated to the keys?
- **hand disbalance** - Are left and right hands similarly loaded?
- **finger balance** - Is each finger suitably loaded? Pinkies less than pointers?
- **finger repeats** - How often are fingers in action consecutively?
- **finger repeats top and bottom** - How often does the same finger need to move from top to bottom row (or vice versa) consecutively?
- **movement pattern** - How often are (near-)neighboring fingers used one after the other?
- **no handswitch after unbalancing key** - How often does no handswitch occur after a hand needed to move away from the home row?
- **unbalancing after neighboring** - How often do unbalancing keys occur consecutively?
- **line changes** - How far (vertically) are consecutive keystrokes of the same hand apart?
- **asymmetric bigrams** - How often are consecutive keystrokes of different hands not symmetrical?
- **manual bigram penalty** - How often do some key-combinations occur that are hard to type but do not fall into the other metrics cases?
- **no handswitch in trigram** - How often does no handswitch happen within a trigram (and have a direction change in between)?
- **irregularity** - How often are the first and the second bigram in a trigram "bad" (wrt. to all bigram metrics)?

## Installation
1. Clone the repository
    ``` sh
    git clone https://github.com/dariogoetz/keyboard_layout_optimizer.git --recurse-submodules
    ```
1. Build the binaries (add `CC=gcc` in the beginning if `cc` is not installed, but `gcc` is)
    ``` sh
    cargo build --release
    ```
    The binaries are then located under `target/release`.
1. Generate documentation with
   ``` sh
   cargo doc
   ```

## Usage
### Specifying Layouts
Some binaries expect layouts as commandline arguments. These layouts are represented as strings specifying the keys of the layout from left to right, top to bottom, i.e. it starts on the top left of the keyboard and lists each letter of the base layer going to the right in the same row. After that the letters of the next row follow, again from left to right.

Whitespace is allowed and will be ignored.

Only those keys shall be specified that are not marked as "fixed" in the layout configuration file "config/keyboard/standard.yml" (usually 32 keys).

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
- cost associated to accessing each layer

Alternatively to `standard.yml`, there are variants for split/ortho keyboards
(`ortho.yml` - a generic ortholinear split keyboard, `moonlander.yml` - the ZSA moonlander
keyboard, `crkbd.yml` - the corne aka. crkbd split keyboard) and variants based on US and UK QWERTY
base layouts instead of neo (`standard_qwerty_uk.yml` and `standard_qwerty_us.yml`).

##### `config/evaluation/default.yml`
This file contains configuration parameters for all available evaluation metrics, filenames of prepared ngram data to use, and parameters specifying the behavior of post-processing the ngram data for a given layout.

There is also an alternative evaluation config file named `arnebab.yml` that aims to stay closer to ArneBab's original
evaluator.

### Layout Optimization Binary
The available optimize-binaries include `optimize_abc.rs`, `optimize_genetic.rs`, and `optimize_sa.rs`.
If run without any commandline parameters, they start with a random layout or a collection of random layouts and optimize from there. With commandline options, a "starting layout" can be specified or a list of keys that shall not be permutated (if no starting layout is given, fixed keys relate to the [Neo2](https://neo-layout.org/) layout).
Optional commandline parameters can be explored with the `-h` option.

Example for a never ending search (appends solutions to a file `found_solutions.txt` and publishes them to https://keyboard-layout-optimizer.herokuapp.com):

``` sh
RUST_LOG=INFO ./target/release/optimize_genetic --run-forever --append-solutions-to "found_solutions.txt" --publish-as "<your name>"
```

#### Optimization Algorithms
- To optimize a preexisting layout while keeping it similar to the original, [Artificial Bee Colony](#artificial-bee-colony-optimize_abcrs) optimization and [Genetic optimization](#genetic-algorithm-optimize_geneticrs) seem to be best suited.
- [Simulated Annealing](#simulated-annealing-optimizesars) produces the best layouts from scratch.

##### Artificial Bee Colony (`optimize_abc.rs`)
Currently, a few of the options available in the other binaries are not yet implemented for this optimization.

Example of an optimization (starting from a random layout, fixing "," and "."):
``` sh
RUST_LOG=INFO ./target/release/optimize_abc -f ",."
```

##### Genetic Algorithm (`optimize_genetic.rs`)
Example (starting from Bone layout, fixing "," and "."):
``` sh
RUST_LOG=INFO ./target/release/optimize_genetic -s "jduax phlmwqß ctieo bnrsg fvüäö yz,.k" -f ",."
```

##### Simulated Annealing (`optimize_sa.rs`)
An explanation of Simulated Annealing can be found [here](https://en.wikipedia.org/wiki/Simulated_annealing/).

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
* `abc.yml`
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
The project includes several binaries within the `evolve_keyboard_layout` crate:
1. `plot` - Plots six layers (neo-layouts have six layers) of a specified layout
1. `evaluate` - Evaluates a specified layout and prints a summary of the various metrics to stdout
1. `optimize_abc` - Starts an optimization heuristic to find a good layout (artificial bee colony algorithm)
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
