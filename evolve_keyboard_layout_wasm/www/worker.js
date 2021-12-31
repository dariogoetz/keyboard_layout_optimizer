importScripts("https://unpkg.com/comlink/dist/umd/comlink.js")
;
// instantiate wasm module
let wasm_import = import("evolve-keyboard-layout-wasm")
let unigram_import = import('../../corpus/arne_no_special/1-grams.txt')
let bigram_import = import('../../corpus/arne_no_special/2-grams.txt')
let trigram_import = import('../../corpus/arne_no_special/3-grams.txt')


const evaluator = {

    wasm: null,
    unigrams: null,
    bigrams: null,
    trigrams: null,
    ngramProvider: null,
    layoutEvaluator: null,
    layoutOptimizer: null,

    init () {
        return Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])
            .then((imports) => {
                this.wasm = imports[0]
                this.unigrams = imports[1].default
                this.bigrams = imports[2].default
                this.trigrams = imports[3].default
            })
    },

    initNgramProvider (ngramType, evalParams, corpusText) {
        if (ngramType === 'prepared') {
            this.ngramProvider = this.wasm.NgramProvider.with_frequencies(
                evalParams,
                this.unigrams,
                this.bigrams,
                this.trigrams
            )
        } else if (ngramType === 'from_text') {
            this.ngramProvider = this.wasm.NgramProvider.with_text(
                evalParams,
                corpusText
            )
        }
    },

    initLayoutEvaluator (layoutConfig, evalParams) {
        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
            layoutConfig,
            evalParams,
            this.ngramProvider,
        )
    },

    initLayoutOptimizer (layout, fixed_chars, optParams) {
        this.layoutOptimizer = this.wasm.LayoutOptimizer.new(
            layout,
            optParams,
            this.layoutEvaluator,
            fixed_chars,
            true,
        )

        return this.layoutOptimizer.parameters()
    },

    optimizationStep () {
        return this.layoutOptimizer.step()
    },

    evaluateLayout (layout) {
        let res = this.layoutEvaluator.evaluate(layout)
        res.layout = layout
        return res
    },

    permutableKeys () {
        return this.layoutEvaluator.permutable_keys()
    }
}

Comlink.expose(evaluator)

