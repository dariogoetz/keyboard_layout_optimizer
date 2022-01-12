importScripts("https://unpkg.com/comlink/dist/umd/comlink.js")

const evaluator = {
    wasm: null,
    ngramProvider: null,
    layoutEvaluator: null,
    layoutOptimizer: null,

    init() {
        return import("evolve-keyboard-layout-wasm")
            .then((wasm) => {
                this.wasm = wasm
            })
    },

    async initNgramProvider(ngramType, evalParams, ngramData) {
        if (ngramType === 'prepared') {
            let unigrams = await import(`../../corpus/${ngramData}/1-grams.txt`)
                .then((ngrams) => ngrams.default)
            let bigrams = await import(`../../corpus/${ngramData}/2-grams.txt`)
                .then((ngrams) => ngrams.default)
            let trigrams = await import(`../../corpus/${ngramData}/3-grams.txt`)
                .then((ngrams) => ngrams.default)

            this.ngramProvider = this.wasm.NgramProvider.with_frequencies(
                evalParams,
                unigrams,
                bigrams,
                trigrams
            )
        } else if (ngramType === 'from_text') {
            this.ngramProvider = this.wasm.NgramProvider.with_text(
                evalParams,
                ngramData
            )
        }
    },

    initLayoutEvaluator(layoutConfig, evalParams) {
        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
            layoutConfig,
            evalParams,
            this.ngramProvider,
        )
    },

    async saOptimize(layout, fixed_chars, optParamsStr, initCallbacks, setMaxStepNr, setCurrentStepNr, setNewBest) {
        // Needed to make the callbacks work in Firefox.
        // In other browsers (for example in Chromium or Midori), this isn't necessary.
        // In those browsers, the whole function can be turned into a syncronous one.
        await initCallbacks()
        let optLayout = this.wasm.sa_optimize(
            layout,
            optParamsStr,
            this.layoutEvaluator,
            fixed_chars,
            true,
            setMaxStepNr,
            setCurrentStepNr,
            setNewBest,
        )

        return optLayout
    },

    initGenLayoutOptimizer(layout, fixed_chars, optParamsStr) {
        this.layoutOptimizer = this.wasm.LayoutOptimizer.new(
            layout,
            optParamsStr,
            this.layoutEvaluator,
            fixed_chars,
            true,
        )

        return this.layoutOptimizer.parameters()
    },

    genOptimizationStep() {
        return this.layoutOptimizer.step()
    },

    evaluateLayout(layout) {
        let res = this.layoutEvaluator.evaluate(layout)
        res.layout = layout
        return res
    },

    permutableKeys() {
        return this.layoutEvaluator.permutable_keys()
    },
}

Comlink.expose(evaluator)

