import config_standard_keyboard from '../../config/standard_keyboard.yml'
import config_ortho from '../../config/ortho.yml'
import eval_params from '../../config/evaluation_parameters.yml'

const NKEYS = 32

Vue.component('evaluator-app', {
    template: `
<b-container fluid>

  <h1>Keyboard Layout Evaluator</h1>
  Explore optimized layouts at <a href="https://keyboard-layout-optimizer.herokuapp.com">https://keyboard-layout-optimizer.herokuapp.com</a>
  <hr>

  <b-row>

    <b-col xl="4" lg="6" style="height: 420px">
      <h2>Layout</h2>
      <b-form inline @submit.stop.prevent @submit="evaluateInput">

        <b-form-input v-model="inputLayoutRaw" placeholder="Layout" class="mb-2 mr-sm-2 mb-sm-0" ></b-form-input>
        <keyboard-selector @selected="selectLayoutConfigType"></keyboard-selector>
      </b-form>
      <layout-plot :layout-string="inputLayout" :wasm="wasm" :layout-config="layoutConfig"></layout-plot>

      <b-button :disabled="loading" @click="evaluateInput" variant="primary">
        <div v-if="loading"><b-spinner small></b-spinner> Loading</div>
        <div v-else>Evaluate</div>
      </b-button>

    </b-col>

    <b-col xl="8" lg="6" style="height: 420px">
      <h2>Settings</h2>
      <b-tabs>

        <b-tab title="Evaluation Parameters">
          <config-file :initial-content="evalParams" @saved="updateEvalParams">
        </b-tab>

        <b-tab title="Ngram Settings">
          <ngram-config @selected="updateNgramProviderParams"></ngram-config>
        </b-tab>

        <b-tab title="Keyboard Settings">
          <config-file :initial-content="layoutConfig" @saved="updateLayoutConfig">
        </b-tab>

      </b-tabs>
    </b-col>
  </b-row>

  <hr>

  <b-row>
    <b-col v-for="detail in details" xl="6">
      <layout-button :layout="detail.layout" @remove="removeLayout"></layout-button>
      <layout-details title="Details" :layout-details="detail"></layout-details>
    </b-col>

    <b-col xl="6" v-if="details.length > 0">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
        <b-form-checkbox v-if="!relative" v-model="logscale" inline>logarithmic scale</b-form-checkbox>
      </b-form>
      <layout-barplot :layout-details="details" :relative="relative" :logscale="logscale && !relative" :styles="chartStyles"></layout-barplot>
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
            details: [],
            inputLayoutRaw: null,
            layoutEvaluator: null,
            ngramType: "prepared",
            ngramProvider: null,
            unigrams: null,
            bigrams: null,
            trigrams: null,
            corpusText: null,
            wasm: null,
            evalParams: null,
            layoutConfigType: "standard",
            layoutConfigStandard: config_standard_keyboard,
            layoutConfigOrtho: config_ortho,
            loading: true,
        }
    },
    computed: {
        chartStyles () {
            return {
                height: "600px",
                position: "relative"
            }
        },
        inputLayout () {
            let layoutString = (this.inputLayoutRaw || "").replace(" ", "")
            layoutString = layoutString.toLowerCase()
            return layoutString
        },
        layoutConfig () {
            if (this.layoutConfigType === "standard") {
                return this.layoutConfigStandard
            } else if (this.layoutConfigType === "ortho") {
                return this.layoutConfigOrtho
            }
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

            this.updateNgramProvider()
            this.updateEvaluator()

            this.loading = false
        })
    },
    methods: {
        evaluateInput () {
            let res = this.evaluate(this.inputLayout)
            if (res !== null) {
                this.details.push(res)
            }
        },
        evaluateExisting () {
            const details = []
            const existingDetails = this.details
            // empty this.details, otherwise the updated layouts will be ignored
            this.details = []
            existingDetails.forEach((d) => {
                let res = this.evaluate(d.layout)
                if (res !== null) {
                    details.push(res)
                }
            })
            this.details = details
        },
        evaluate (layout) {
            if (layout.length !== NKEYS) {
                this.$bvToast.toast("Keyboard layout must have 32 (non-whitespace) symbols", {variant: "danger"})
                return null
            }

            if (this.details.filter((d) => d.layout == layout).length > 0) {
                this.$bvToast.toast(`Layout ${layout} is already available`, {variant: "primary"})
                return null
            }

            try {
                this.$bvToast.toast(`Evaluating layout "${layout}"`, {variant: "primary"})
                let res = this.layoutEvaluator.evaluate(layout)
                res.layout = layout
                return res
            } catch(err) {
                this.$bvToast.toast(`Could not generate a valid layout: ${err}`, {variant: "danger"})
                return null
            }
        },
        updateNgramProvider () {
            this.$bvToast.toast(`(Re-)Generating Ngram Provider`, {variant: "primary"})
            this.loading = true
            if (this.ngramType === "prepared") {
                this.ngramProvider = this.wasm.NgramProvider.with_frequencies(
                    this.evalParams,
                    this.unigrams,
                    this.bigrams,
                    this.trigrams
                )
            } else if (this.ngramType === "from_text") {
                this.ngramProvider = this.wasm.NgramProvider.with_text(
                    this.evalParams,
                    this.corpusText
                )
            }
            this.loading = false
        },
        updateEvaluator () {
            this.$bvToast.toast(`(Re-)Generating Evaluator`, {variant: "primary"})
            this.loading = true
            this.layoutEvaluator = this.wasm.LayoutEvaluator.new(
                this.layoutConfig,
                this.evalParams,
                this.ngramProvider
            )
            this.loading = false
        },
        updateEvalParams (evalParams) {
            this.evalParams = evalParams

            this.updateNgramProvider()
            this.updateEvaluator()

            this.evaluateExisting()
        },
        updateNgramProviderParams (ngramType, ngramData) {
            this.ngramType = ngramType
            if (ngramType === "from_text") {
                this.corpusText = ngramData
            }

            this.updateNgramProvider()
            this.updateEvaluator()

            this.evaluateExisting()
        },
        updateLayoutConfig (layoutConfig) {
            if (this.layoutConfigType === "standard") {
                this.layoutConfigStandard = layoutConfig
            } else if (this.layoutConfigType === "ortho") {
                this.layoutConfigOrtho = layoutConfig
            }

            this.updateEvaluator()

            this.evaluateExisting()
        },
        selectLayoutConfigType (layoutConfigType) {
            this.layoutConfigType = layoutConfigType

            this.updateEvaluator()

            this.evaluateExisting()
        },
        removeLayout (layout) {
            this.details = this.details.filter((d) => d.layout !== layout)
        },
    }
})

Vue.component('layout-button', {
    template: `
    <div>
      <b-button-group size="sm" class="mx-1">
        <b-button>{{layout}}</b-button>
        <b-button variant="danger" @click="remove"><b-icon-x-circle-fill></b-button>
      </b-button-group>
    </div>
    `,
    props: {
        layout: { type: String, default: "", required: true },
    },
    methods: {
        remove () {
            this.$emit("remove", this.layout)
        },
    },
})

Vue.component('keyboard-selector', {
    template: `
    <div>
    <b-form inline>
      <label class="mr-sm-2">Keyboard</label>
      <b-form-select v-model="selected" :options="options" @change="emit"></b-form-select>
    </b-form>
    </div>
    `,
    props: {
        defaultSelection: { type: String, default: "standard" },
    },
    data () {
        return {
            selected: this.defaultSelection,
            options: [
                { value: "standard", text: "Standard" },
                { value: "ortho", text: "Ortho" },
            ],
        }
    },
    methods: {
        emit () {
            this.$emit("selected", this.selected)
        }
    },
})

Vue.component('ngram-config', {
    template: `
    <div>
      <b-form-select label="NGram Type" v-model="selected" :options="options" @change="select"></b-form-select>
      <div v-if="selected === 'prepared'">Using prepared ngram frequencies</div>
      <div v-if="selected === 'from_text'">
        <b-form-textarea
          v-model="text"
          placeholder="Text to evaluate layouts on"
          rows="10"
        >
        </b-form-textarea>
        <b-button class="float-right" variant="primary" @click="save">Save</b-button>
      </div>
    </div>
    `,
    props: {
        defaultSelection: { type: String, default: "prepared" },
    },
    data () {
        return {
            selected: this.defaultSelection,
            options: [
                { value: "prepared", text: "Prepared" },
                { value: "from_text", text: "From Text" },
            ],
            text: "",
        }
    },
    methods: {
        select () {
            if (this.selected === "prepared") {
                this.$emit("selected", this.selected, null)
            }
        },
        save () {
            this.$emit("selected", this.selected, this.text)
        },
    },
})

Vue.component('config-file', {
    template: `
    <div>
      <b-form-textarea
        v-model="content"
        rows="15"
        style="font: 400 13px/18px 'Source Code Pro', monospace;"
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
    watch: {
        initialContent () {
            this.content = this.initialContent
        },
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
        layoutConfig: { type: Object, default: null },
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
            this.update()
        },
        layoutConfig () {
            this.update()
        },
    },
    mounted () {
        this.update()
    },
    methods: {
        update () {
            if (this.wasm === null || this.layoutConfig === null) return
            try {
                this.layoutPlotter = this.wasm.LayoutPlotter.new(this.layoutConfig)
            } catch (err) {
                this.$bvToast.toast(`Error plotting the layout: ${err}`, {variant: 'danger'})
            }
            this.plot()
        },
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
