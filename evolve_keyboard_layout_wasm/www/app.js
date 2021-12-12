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
      <b-form inline>
        <b-form-input v-model="layout" placeholder="Layout" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        <b-button :disabled="loading" @click="evaluate" variant="primary">
          <div v-if="loading"><b-spinner small></b-spinner> Loading</div>
          <div v-else>Evaluate</div>
        </b-button>
      </b-form>
      <layout-plot :layout-string="layout" :wasm="wasm"></layout-plot>
      <layout-details v-if="details !== null" title="Details" :layout-details="details"></layout-details>
    </b-col>

    <b-col xl="6">
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
            layout: null,
            layoutEvaluator: null,
            wasm: null,
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
    },
    created () {
        let wasm_import = import("evolve-keyboard-layout-wasm")
        let unigram_import = import('../../1-gramme.arne.no-special.txt')
        let bigram_import = import('../../2-gramme.arne.no-special.txt')
        let trigram_import = import('../../3-gramme.arne.no-special.txt')

        wasm_import.then((wasm) => {
            this.wasm = wasm
        })

        Promise.all([wasm_import, unigram_import, bigram_import, trigram_import])
        .then((imports) => {
            let wasm = imports[0]
            let unigrams = imports[1].default
            let bigrams = imports[2].default
            let trigrams = imports[3].default

            this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
                layout_config,
                eval_params,
                unigrams,
                bigrams,
                trigrams
            )
            this.loading = false
        })
    },
    methods: {
        evaluate () {
            if (this.layout === null) return
            if (this.layout.length !== NKEYS) {
                this.$bvToast.toast("Keyboard layout must have 32 (non-whitespace) symbols", {variant: "danger"})
            }
            let details = this.layoutEvaluator.evaluate(this.layout)
            details.layout = this.layout
            this.details = details
        }
    }
})


Vue.component('layout-plot', {
    template: `
    <pre><code>
{{plotString}}
    </code></pre>
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

            let layoutString = (this.layoutString || "").replace(" ", "")
            const nMissing = NKEYS - layoutString.length
            let layout = layoutString + Array(nMissing + 1).join(this.defaultSymbol)
            this.plotString = this.layoutPlotter.plot(layout, 0)
        },
    },
})

var app = new Vue({
    el: '#app',
})
