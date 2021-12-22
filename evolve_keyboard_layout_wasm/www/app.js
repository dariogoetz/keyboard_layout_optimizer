import config_standard_keyboard from '../../config/standard_keyboard.yml'
import config_ortho from '../../config/ortho.yml'
import config_ortho_bored from '../../config/ortho_bored.yml'

import eval_params from '../../config/evaluation_parameters.yml'
import opt_params from '../../config/optimization_parameters_web.yml'

import Worker from "./worker.js"

const PUBLISH_URL = "https://keyboard-layout-optimizer.herokuapp.com/api"

const LAYOUT_CONFIGS = [
    { key: 'standard', label: 'Standard', config: config_standard_keyboard },
    { key: 'ortho', label: 'Ortho', config: config_ortho },
    { key: 'ortho_bored', label: 'Ortho (bored)', config: config_ortho_bored },
]

function setDifference(setA, setB) {
    var _difference = new Set(setA);
    for (var elem of setB) {
        _difference.delete(elem);
    }
    return _difference;
}

Vue.component('evaluator-app', {
    template: `
<b-container fluid>

  <h1>Keyboard Layout Evaluator</h1>
  Explore optimized layouts at <a href="https://keyboard-layout-optimizer.herokuapp.com">https://keyboard-layout-optimizer.herokuapp.com</a>
  <hr>

  <b-row>

    <b-col xl="4" lg="6" style="height: 450px">
      <h2>Layout</h2>
      <b-form inline @submit.stop.prevent @submit="evaluateInput">
        <b-form-input v-model="inputLayoutRaw" :state="inputLayoutValid" placeholder="Layout" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        <keyboard-selector @selected="selectLayoutConfigType"></keyboard-selector>
        <b-form-invalid-feedback>{{invalidInputFeedback}}</b-form-invalid-feedback>
      </b-form>
      <layout-plot :layout-string="inputLayout" :wasm="wasm" :layout-config="layoutConfig" :permutableKeys="permutableKeys"></layout-plot>

      <b-button :disabled="loading > 0" @click="evaluateInput" variant="primary">
        <div v-if="loading > 0"><b-spinner small></b-spinner> Loading</div>
        <div v-else>Evaluate</div>
      </b-button>

      <b-button-group class="float-right">
        <b-button :disabled="optStep > 0 || loading > 0" @click="optimizeInput" variant="secondary">
          <div v-if="optStep > 0 || loading > 0">
            <b-spinner small></b-spinner>
            <span v-if="optStep > 0">Iteration {{optStep}}/{{optTotalSteps}}</span>
            <span v-else>Loading</span>
          </div>
          <div v-else>Optimize</div>
        </b-button>
        <b-button v-if="optStep > 0" @click="optCancelRequest" variant="danger"><b-icon-x-circle-fill /></b-button>
      </b-button-group>

    </b-col>

    <b-col xl="8" lg="6" style="height: 450px">
      <h2>Settings</h2>
      <b-tabs>

        <b-tab title="Evaluation">
          <config-file :initial-content="evalParamsStr" @saved="updateEvalParams">
        </b-tab>

        <b-tab title="Ngrams">
          <ngram-config @selected="updateNgramProviderParams"></ngram-config>
        </b-tab>

        <b-tab title="Keyboard">
          <config-file :initial-content="layoutConfig" @saved="updateLayoutConfig">
        </b-tab>

        <b-tab title="Optimization">
      <b-form inline @submit.stop.prevent @submit="evaluateInput">
          <label class="mr-sm-2">Fixed Keys</label>
          <b-form-input v-model="optFixed" placeholder="Fixed Keys" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        </b-form>
          <config-file :initial-content="optParamsStr" @saved="updateOptParams">
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
        let layoutConfigs = {}
        LAYOUT_CONFIGS.forEach((c) => {
            layoutConfigs[c.key] = c.config
        })
        return {
            details: [],
            inputLayoutRaw: null,
            showInputValidState: false,
            wasm: null,
            worker: null,
            ngramType: "prepared",
            corpusText: null,
            evalParamsStr: null,
            permutableKeys: null,
            optParamsStr: null,
            optParams: null,
            selectedLayoutConfig: "standard",
            layoutConfigs,
            loading: 1,
            optStep: 0,
            optTotalSteps: 0,
            optFixed: ",.",
            optCancel: false,
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
            return this.layoutConfigs[this.selectedLayoutConfig]
        },

        invalidInputFeedback () {
            let permutableKeys = new Set(this.permutableKeys)
            let givenKeys = new Set()
            let duplicates = new Set()

            for (let i = 0; i < this.inputLayout.length; i++) {
                let c = this.inputLayout.charAt(i)
                if (givenKeys.has(c)) {
                    duplicates.add(c)
                } else {
                    givenKeys.add(c)
                }
            }

            duplicates = Array.from(duplicates).sort()
            let missing = Array.from(setDifference(permutableKeys, givenKeys)).sort()
            let unknown = Array.from(setDifference(givenKeys, permutableKeys)).sort()

            if (duplicates.length === 0 && missing.length === 0 && unknown.length === 0) {
                return null
            }

            let msg = ""
            if (duplicates.length) {
                msg += `Duplicates: "${duplicates.join('')}". `
            }
            if (missing.length) {
                msg += `Missing: "${missing.join('')}". `
            }
            if (unknown.length) {
                msg += `Unknown: "${unknown.join('')}". `
            }

            return msg
        },

        inputLayoutValid () {
            if (!this.showInputValidState) {
                return null
            } else if (this.invalidInputFeedback === null) {
                return true
            } else {
                return false
            }
        }

    },

    async created () {
        this.evalParamsStr = eval_params
        this.optParamsStr = opt_params

        this.wasm = await import("evolve-keyboard-layout-wasm")

        this.worker = Comlink.wrap(new Worker('worker.js'))
        await this.worker.init()

        await this.initNgramProvider()
        await this.initLayoutEvaluator()

        // reduce initial value of this.loading
        this.loading -= 1
    },

    methods: {
        async evaluateInput () {
            this.showInputValidState = true
            // check if the current layout is already available in this.details
            let existing = this.details.filter((d) => d.layout == this.inputLayout)
            if (existing.length > 0) {
                this.$bvToast.toast(`Layout '${this.inputLayout}' is already available`, {variant: "primary"})
            } else {
                try {
                    let details = await this.evaluate(this.inputLayout)
                    this.details.push(details)
                    this.showInputValidState = false
                } catch (err) {
                    console.error(err)
                }
            }
        },

        async evaluateExisting () {
            let promises = []
            this.details.forEach((d) => {
                let promise = this.evaluate(d.layout)
                promises.push(promise)
            })

            try {
                let details = await Promise.all(promises)
                this.details = details
            } catch (err) {
                console.error(err)
            }
        },

        async evaluate (layout) {
            let promise = new Promise(async (resolve, reject) => {

                if (this.inputLayoutValid !== null && !this.inputLayoutValid) {
                    this.$bvToast.toast("Could not evaluate Layout: " + this.invalidInputFeedback, {variant: "danger"})
                    return
                }

                this.$bvToast.toast(`Evaluating layout "${layout}"`, {variant: "primary"})
                this.loading += 1
                try {
                    let res = await this.worker.evaluateLayout(layout)
                    res.layout = layout
                    this.loading -= 1
                    resolve(res)
                } catch (err) {
                    this.$bvToast.toast(`Could not generate a valid layout: ${err}`, {variant: "danger"})
                    this.loading -= 1
                    reject(err)
                }
            })
            return promise
        },

        async initNgramProvider () {
            // this.$bvToast.toast(`(Re-)Generating Ngram Provider`, {variant: "primary"})
            this.loading += 1
            await this.worker.initNgramProvider(this.ngramType, this.evalParamsStr, this.corpusText)
            this.loading -= 1
        },

        async initLayoutEvaluator () {
            // this.$bvToast.toast(`(Re-)Generating Evaluator`, {variant: "primary"})
            this.loading += 1
            await this.worker.initLayoutEvaluator(this.layoutConfig, this.evalParamsStr)
            this.permutableKeys = await this.worker.permutableKeys()
            this.loading -= 1
        },

        async updateEvalParams (evalParamsStr) {
            this.evalParamsStr = evalParamsStr

            await this.initNgramProvider()
            await this.initLayoutEvaluator()
            await this.evaluateExisting()
        },

        updateOptParams (optParamsStr) {
            this.optParamsStr = optParamsStr
        },

        async updateNgramProviderParams (ngramType, ngramData) {
            this.ngramType = ngramType
            if (ngramType === "from_text") {
                this.corpusText = ngramData
            }

            await this.initNgramProvider()
            await this.initLayoutEvaluator()
            await this.evaluateExisting()
        },

        async updateLayoutConfig (layoutConfig) {
            this.layoutConfigs[this.selectedLayoutConfig] = layoutConfig

            await this.initLayoutEvaluator()
            await this.evaluateExisting()

        },

        async selectLayoutConfigType (selectedLayoutConfig) {
            this.selectedLayoutConfig = selectedLayoutConfig

            await this.initLayoutEvaluator()
            await this.evaluateExisting()
        },

        removeLayout (layout) {
            this.details = this.details.filter((d) => d.layout !== layout)
        },

        async optimizeInput () {
            // check if given layout_str is valid
            try {
                this.showInputValidState = true
                await this.evaluate(this.inputLayout)
                this.showInputValidState = false
            } catch (err) {
                this.optStep = 0
                this.optCancel = false
                return
            }

            this.optParams = await this.worker.initLayoutOptimizer(
                this.inputLayout,
                this.optFixed,
                this.optParamsStr
            )

            this.optTotalSteps = this.optParams.generation_limit
            this.optStep = 1
            this.optCancel = false

            this.$bvToast.toast(`Starting optimization of ${this.inputLayout}`, {variant: "primary"})
            let res
            do {
                res = await this.worker.optimizationStep()
                if (res !== null) {
                    if (res.layout !== this.inputLayout) {
                        this.$bvToast.toast(`New layout found: ${res.layout}`, {variant: "primary"})
                        this.inputLayoutRaw = res.layout
                    }
                    this.optStep += 1
                }
            } while (res !== null && !this.optCancel)

            this.$bvToast.toast("Optimization finished", {variant: "primary"})
            this.optStep = 0
            this.optCancel = false
        },

        optCancelRequest () {
            this.optCancel = true
            this.$bvToast.toast("Stopping optimization", {variant: "primary"})
        },
    }
})

Vue.component('layout-button', {
    template: `
      <div>
        <b-button-group size="sm" class="mx-1">
          <b-button disabled variant="outline-dark">{{layout}}</b-button>
          <b-button variant="secondary" @click="showModal = !showModal">Publish</b-button>
          <b-button variant="danger" @click="remove"><b-icon-x-circle-fill /></b-button>
        </b-button-group>
        <b-modal v-model="showModal" title="Publish Layout" @ok="publish">
          <label class="mr-sm-2">Publish Name</label>
          <b-form-input v-model="publishName" :state="nameState" placeholder="Name to publish result under" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        </b-modal>
      </div>
    `,
    props: {
        layout: { type: String, default: "", required: true },
    },
    data () {
        return {
            publishName: null,
            showNameState: false,
            showModal: false,
        }
    },
    computed: {
        nameState () {
            if (!this.showNameState) {
                return null
            } else if (this.publishName === null || this.publishName.length === 0) {
                return false
            } else {
                return true
            }
        },
    },
    methods: {
        remove () {
            this.$emit("remove", this.layout)
        },
        async publish (bvModalEvt) {
            this.showNameState = true
            if (!this.nameState) {
                bvModalEvt.preventDefault()
                return
            }
            try {
                let res = await fetch(PUBLISH_URL, {
                    method: "POST",
                    headers: {
                      'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ layout: this.layout, published_by: this.publishName })
                })
                let resData = await res.json()
                if (resData.published_by !== this.publishName) {
                    this.$bvToast.toast(`Layout had already been published: Cost: ${resData.total_cost.toFixed(2)}`, {variant: 'warning'})
                } else {
                    this.$bvToast.toast(`Successfully published layout: Cost: ${resData.total_cost.toFixed(2)}`, {variant: 'primary'})
                }
            } catch (err) {
                this.$bvToast.toast(`Error while publishing layout: ${err}`, {variant: 'danger'})
            }
        }
    },
})

Vue.component('keyboard-selector', {
    template: `
    <b-form inline>
      <label class="mr-sm-2">Keyboard</label>
      <b-form-select v-model="selected" :options="options" @change="emit"></b-form-select>
    </b-form>
    `,
    props: {
        defaultSelection: { type: String, default: "standard" },
    },
    data () {
        let options = []
        LAYOUT_CONFIGS.forEach(c => {
            options.push({ value: c.key, text: c.label })
        })
        return {
            selected: this.defaultSelection,
            options,
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
        style="font: 400 13px/18px 'DejaVuSansMonoBook', monospace;"
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
    <pre style="overflow-y: hidden"><code v-html="plotString"></code></pre>
`,
    props: {
        layoutString: { type: String, default: "" },
        defaultSymbol: { type: String, default: "." },
        wasm: { type: Object, default: null },
        layoutConfig: { type: Object, default: null },
        permutableKeys: { type: Array, default: null },
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
        permutableKeys () {
            this.update()
        },
    },
    mounted () {
        this.update()
    },
    methods: {
        update () {
            if (this.wasm === null || this.layoutConfig === null || this.permutableKeys === null) return
            try {
                this.layoutPlotter = this.wasm.LayoutPlotter.new(this.layoutConfig)
            } catch (err) {
                this.$bvToast.toast(`Error plotting the layout: ${err}`, {variant: 'danger'})
            }
            this.plot()
        },
        plot () {
            if (this.layoutPlotter === null) return ""

            const nMissing = this.permutableKeys.length - this.layoutString.length
            if (nMissing < 0) {
                // this.$bvToast.toast(`Too many symbols given (${this.layoutString.length} > ${this.permutableKeys.length})`, {variant: "danger"})
                return
            }
            let layout = this.layoutString + Array(nMissing + 1).join(this.defaultSymbol)
            try {
                this.plotString = this.layoutPlotter.plot(layout, 0)
            } catch (err) {
                // this.$bvToast.toast(`Could not plot layout: ${err}`, {variant: "danger"})
                return
            }
        },
    },
})

var app = new Vue({
    el: '#app',
})
