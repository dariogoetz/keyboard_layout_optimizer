importScripts("https://unpkg.com/comlink/dist/umd/comlink.js")
;
// instantiate wasm module
let wasm_import = import("evolve-keyboard-layout-wasm")
let unigram_import = import('../../1-gramme.arne.no-special.txt')
let bigram_import = import('../../2-gramme.arne.no-special.txt')
let trigram_import = import('../../3-gramme.arne.no-special.txt')


const evaluator = {

    wasm: null,
    unigrams: null,
    bigrams: null,
    trigrams: null,
    ngramProvider: null,
    layoutEvaluator: null,

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

    evaluateLayout (layout) {
        let res = this.layoutEvaluator.evaluate(layout)
        res.layout = layout
        return res
    },
}

Comlink.expose(evaluator)

