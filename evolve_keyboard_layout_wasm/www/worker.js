// Create promise to handle Worker calls whilst
// module is still initialising
let initResolve;
let ready = new Promise((resolve) => {
    initResolve = resolve;
})

// instantiate wasm module
let wasm_import = import("evolve-keyboard-layout-wasm")
let unigram_import = import('../../1-gramme.arne.no-special.txt')
let bigram_import = import('../../2-gramme.arne.no-special.txt')
let trigram_import = import('../../3-gramme.arne.no-special.txt')

let ngramProvider;
let layoutEvaluator;



class Evaluator {
    constructor (wasm, unigrams, bigrams, trigrams) {
        this.wasm = wasm
        this.unigrams = unigrams
        this.bigrams = bigrams
        this.trigrams = trigrams

        this.ngramProvider = null
        this.layoutEvaluator = null
    }

    test () {
        return "test"
    }

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
    }

    initLayoutEvaluator (layoutConfig, evalParams) {
        this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
            layoutConfig,
            evalParams,
            this.ngramProvider,
        )
    }

    evaluateLayout (layout) {
        let res = this.layoutEvaluator.evaluate(layout)
        res.layout = layout
        return res
    }
}


// listen to messages sent from main thread
self.addEventListener('message', function(event) {
    const { eventType, eventData, eventId } = event.data;


    if (eventType === "initialise") {
        Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])
            .then((imports) => {
                let wasm = imports[0]
                let unigrams = imports[1].default
                let bigrams = imports[2].default
                let trigrams = imports[3].default
                let evaluator = new Evaluator(wasm, unigrams, bigrams, trigrams)
                initResolve(evaluator)

                // Send back initialised message to main thread
                self.postMessage({
                    eventType: "initialised",
                    eventData: Object.getOwnPropertyNames(Object.getPrototypeOf(evaluator))
                });

            });
    } else if (eventType === "call") {
        ready
            .then((evaluator) => {
                const method = evaluator[eventData.method].bind(evaluator);
                const result = method.apply(null, eventData.arguments);
                self.postMessage({
                    eventType: "result",
                    eventData: result,
                    eventId: eventId
                });
            })
            .catch((error) => {
                self.postMessage({
                    eventType: "error",
                    eventData: "An error occured executing WASM instance function: " + error.toString(),
                    eventId: eventId
                });
            })
    }





}, false)
