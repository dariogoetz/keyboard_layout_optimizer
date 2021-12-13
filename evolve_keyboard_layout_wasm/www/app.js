import layout_config from '../../config/standard_keyboard.yml'
import eval_params from '../../config/evaluation_parameters.yml'

const NKEYS = 32

Vue.component('evaluator-app', {
    template: `
<b-container>

  <h1>Keyboard Layout Evaluator</h1>
  <hr>

  <b-row>
    <b-col xl="6">
      <b-form inline @submit.stop.prevent @submit="evaluate">
        <b-form-input v-model="layoutRaw" placeholder="Layout" class="mb-2 mr-sm-2 mb-sm-0" ></b-form-input>
        <b-button :disabled="loading" @click="evaluate" variant="primary">
          <div v-if="loading"><b-spinner small></b-spinner> Loading</div>
          <div v-else>Evaluate</div>
        </b-button>
      </b-form>
      <layout-plot :layout-string="layout" :wasm="wasm"></layout-plot>
      <layout-details v-if="details !== null" title="Details" :layout-details="details"></layout-details>
    </b-col>
    <b-col xl="6">
        <config-file :initial-content="evalParams" @saved="updateEvalParams">
    </b-col>
  </b-row>
  <b-row>
    <b-col v-if="details !== null" xl="6">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
        <b-form-checkbox v-if="!relative" v-model="logscale" inline>logarithmic scale</b-form-checkbox>
      </b-form>
      <layout-barplot :layout-details="detailsArray" :relative="relative" :logscale="logscale && !relative" :styles="chartStyles"></layout-barplot>
    </b-col>
  </b-row>

</b-container>
`,
    props: {
        relative: { type: Boolean, default: false },
        logscale: { type: Boolean, default: false },
    },
    data () {
        return {
            details: null,
            layoutRaw: null,
            layoutEvaluator: null,
            frequenciesNgramProvider: null,
            unigrams: null,
            bigrams: null,
            trigrams: null,
            wasm: null,
            evalParams: null,
            loading: true,
        }
    },
    computed: {
        detailsArray () {
            if (this.details === null) return []
            return [this.details]
        },
        chartStyles () {
            return {
                height: "600px",
                position: "relative"
            }
        },
        layout () {
            let layoutString = (this.layoutRaw || "").replace(" ", "")
            layoutString = layoutString.toLowerCase()
            return layoutString
        },
    },
    created () {
        this.evalParams = eval_params

        let wasm_import = import("evolve-keyboard-layout-wasm")
        let unigram_import = import('../../1-gramme.arne.no-special.txt')
        let bigram_import = import('../../2-gramme.arne.no-special.txt')
        let trigram_import = import('../../3-gramme.arne.no-special.txt')

        wasm_import.then((wasm) => {
            this.wasm = wasm
        })

        Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])
        .then((imports) => {
            this.unigrams = imports[1].default
            this.bigrams = imports[2].default
            this.trigrams = imports[3].default

            this.updateFrequenciesNgramProvider()
            this.updateEvaluator()

            this.loading = false
        })
    },
    methods: {
        evaluate () {
            if (this.layout.length !== NKEYS) {
                this.$bvToast.toast("Keyboard layout must have 32 (non-whitespace) symbols", {variant: "danger"})
                return
            }
            try {
                this.$bvToast.toast(`Evaluating layout "${this.layout}"`, {variant: "primary"})
                let details = this.layoutEvaluator.evaluate(this.layout)
                details.layout = this.layout
                this.details = details
            } catch(err) {
                this.$bvToast.toast(`Could not generate a valid layout: ${err}`, {variant: "danger"})
                return
            }
        },
        updateFrequenciesNgramProvider () {
            this.$bvToast.toast(`(Re-)Generating Ngram Provider`, {variant: "primary"})
            this.loading = true
            this.details = null
            this.frequenciesNgramProvider = this.wasm.NgramProvider.with_frequencies(
                this.evalParams,
                this.unigrams,
                this.bigrams,
                this.trigrams
            )
            this.loading = false
        },
        updateEvaluator () {
            this.$bvToast.toast(`(Re-)Generating Evaluator`, {variant: "primary"})
            this.loading = true
            this.details = null
            this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
                layout_config,
                this.evalParams,
                this.frequenciesNgramProvider
            )
            this.loading = false
        },
        updateEvalParams (evalParams) {
            let hadDetails = this.details !== null
            this.evalParams = evalParams

            this.updateFrequenciesNgramProvider()
            this.updateEvaluator()

            if (hadDetails) {
                this.evaluate()
            }
        },
    }
})


Vue.component('config-file', {
    template: `
    <div>
        <b-form-textarea
          v-model="content"
          rows="15"
          style="font: 400 13px/18px 'Source Code Pro', monospace;"
          no-resize
        ></b-form-textarea>
        <b-button class="float-right" variant="primary" @click="save">Save</b-button>
    </div>
    `,
    props: {
        initialContent: { type: String, default: "" },
    },
    data () {
        return {
            content: this.initialContent
        }
    },
    methods: {
        save () {
            this.$emit("saved", this.content)
        },
    },
})


Vue.component('layout-plot', {
    template: `
    <pre><code v-html="plotString"></code></pre>
`,
    props: {
        layoutString: { type: String, default: "" },
        defaultSymbol: { type: String, default: "." },
        wasm: { type: Object, default: null },
    },
    data () {
        return {
            plotString: null,
            layoutPlotter: null,
        }
    },
    watch: {
        layoutString () {
            this.plot()
        },
        wasm () {
            if (this.wasm === null) return
            this.layoutPlotter = this.wasm.LayoutPlotter.new(layout_config)
            this.plot()
        },
    },
    methods: {
        plot () {
            if (this.layoutPlotter === null) return ""

            const nMissing = NKEYS - this.layoutString.length
            if (nMissing < 0) {
                this.$bvToast.toast(`Too many symbols given (${this.layoutString.length} > ${NKEYS})`, {variant: "danger"})
                return
            }
            let layout = this.layoutString + Array(nMissing + 1).join(this.defaultSymbol)
            try {
                this.plotString = this.layoutPlotter.plot(layout, 0)
            } catch (err) {
                this.$bvToast.toast(`Could not plot layout: ${err}`, {variant: "danger"})
                return
            }
        },
    },
})

var app = new Vue({
    el: '#app',
})
