import config_standard_keyboard from '../../../config/keyboard/standard.yml'
import config_ortho from '../../../config/keyboard/ortho.yml'
import config_moonlander from '../../../config/keyboard/moonlander.yml'
import config_crkbd from '../../../config/keyboard/crkbd.yml'
import config_lily58 from '../../../config/keyboard/lily58.yml'
import config_ansi from '../../../config/keyboard/standard_qwerty_ansi.yml'

import eval_params from '../../../config/evaluation/default.yml'
import genevo_opt_params from '../../../config/optimization/genetic_web.yml'
import sa_opt_params from '../../../config/optimization/sa_web.yml'

import Worker from "./worker.js"

const LAYOUT_CONFIGS = {
  standard: config_standard_keyboard,
  ortho: config_ortho,
  moonlander: config_moonlander,
  crkbd: config_crkbd,
  lily58: config_lily58,
  ansi: config_ansi,
}

const OPTIMIZATION_ALGORITHM_PARAMS = {
  "genevo": genevo_opt_params,
  "simulated_annealing": sa_opt_params,
}

const PUBLISH_URL = "https://keyboard-layout-optimizer.fly.dev/api"

const NGRAMS = [
  { key: 'deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4', label: 'Blend (deu/eng 60/40)', description: 'Ngram frequencies from various German and English corpora in relation 60 to 40. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_wiki_0.6_eng_wiki_0.4', label: 'Wikipedia (deu/eng 60/40)', description: 'Ngram frequencies from German (2021) and English (2016) Wikipedia in relation 60 to 40. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_wiki_1m', label: 'Wikipedia (deu)', description: 'Ngram frequencies from German Wikipedia 2021. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'eng_wiki_1m', label: 'Wikipedia (eng)', description: 'Ngram frequencies from English Wikipedia 2016. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_web_0.6_eng_web_0.4', label: 'Wikipedia (deu/eng 60/40)', description: 'Ngram frequencies from German (2021) and English (2016) Wikipedia in relation 60 to 40. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_web_1m', label: 'Web-public (deu)', description: 'Ngram frequencies from German "Web-public 2019". Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'eng_web_1m', label: 'Web-public (eng)', description: 'Ngram frequencies from English "Web-public 2018". Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_mixed_0.6_eng_news_typical_0.4', label: 'Mixed/News typical (deu/eng 60/40)', description: 'Ngram frequencies from German "Mixed Typical (2011)" and English "News Typical (2016)" in relation 60 to 40. Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'deu_mixed_1m', label: 'Mixed Typical (deu)', description: 'Ngram frequencies from German "Mixed 2011". Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'eng_news_typical_1m', label: 'News Typical (eng)', description: 'Ngram frequencies from English "News Typical 2016". Sourced from <a href="https://wortschatz.uni-leipzig.de/en/download">Wortschatz of Uni Leipzig</a>.' },
  { key: 'irc_neo', label: '#neo - IRC', description: 'Ngram frequencies from the #neo IRC channel. Mostly in German.' },
  { key: 'arne_no_special', label: 'ArneBab', description: 'Ngram frequencies used in ArneBabs optimizer. Sourced from <a href="https://hg.sr.ht/~arnebab/evolve-keyboard-layout">ArneBabs optimizer</a>.' },
]
const DEFAULT_NGRAM = 'deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4'

function setDifference(setA, setB) {
  var _difference = new Set(setA);
  for (var elem of setB) {
    _difference.delete(elem);
  }
  return _difference;
}

Vue.use(VueCodemirror)

Vue.component('evaluator-app', {
  template: `
<b-container fluid>

  <h1>Keyboard Layout Evaluator</h1>
  Explore optimized layouts at <a href="https://keyboard-layout-optimizer.fly.dev">https://keyboard-layout-optimizer.fly.dev</a>
  <hr>

  <b-row>

    <b-col xl="4" lg="6">
      <h2>Layout</h2>
      <b-button class="mb-2" size="sm" @click="setInput('zyuaqpbmlfjßcsieodtnrhvxüäöwg,.k')">noted</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('zguaqwbdlyjßcsieomntrhvxüäöpf,.k')">miners</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('zluaqwbdgyjßcrieomntshvxüäöpf,.k')">mine</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('jduaxphlmwqßctieobnrsgfvüäöyz,.k')">bone</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('xvlcwkhgfqyßuiaeosnrtdüöäpzbm,.j')">neo2</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('k.o,yvgclfzßhaeiudtrnsxqäüöbpwmj')">koy</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('kuü.ävgcljfßhieaodtrnsxyö,qbpwmz')">AdNW</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('qwertzuiopüßasdfghjklöyxcvbnm,.ä')">qwertz</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('qwfpgjluyüößarstdhneiozxcvbkm,.ä')">colemak</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('qdrwbjfupüößashtgyneoizxmcvkl,.ä')">workman</b-button>
      <b-button class="mb-2" size="sm" @click="setInput('qwertyuiop[]asdfghjkl;\\\'zxcvbnm,./')">qwerty (ANSI)</b-button>
      <b-button class="mb-2" size="sm" @click="randomInput(true)">random (std)</b-button>
      <b-button class="mb-2" size="sm" @click="randomInput(false)">random</b-button>

      <b-form inline @submit.stop.prevent @submit="evaluateInput">
        <b-form-input v-model="inputLayoutRaw" :state="inputLayoutValid" placeholder="Enter Keyboard Layout..." class="mb-2 mr-sm-2 mb-sm-0" autofocus></b-form-input>
        <keyboard-selector @selected="selectLayoutConfigType"></keyboard-selector>
        <b-form-invalid-feedback>{{invalidInputFeedback}}</b-form-invalid-feedback>
      </b-form>
      <layout-plot :layout-string="inputLayout" :wasm="wasm" :layout-config="layoutConfig" :permutableKeys="permutableKeys"></layout-plot>

      <b-button :disabled="loading > 0 || saOptimizationOngoing" @click="evaluateInput" variant="primary">
        <div v-if="loading > 0"><b-spinner small></b-spinner> Loading</div>
        <div v-else>Evaluate</div>
      </b-button>

      <b-button-group class="float-right">
        <b-button :disabled="optStep >= 0 || loading > 0" @click="startOptimization" variant="secondary">
          <div v-if="optStep >= 0 || loading > 0">
            <b-spinner small></b-spinner>
            <span v-if="optStep >= 0">
                <span v-if="optMode==='simulated_annealing'">Iteration {{optStep}} // {{temperatureStr}}°</span>
                <span v-if="optMode==='genevo'">Iteration {{optStep}}/{{optTotalSteps}}</span>
            </span>
            <span v-else>Loading</span>
          </div>
          <div v-else>Optimize</div>
        </b-button>
        <b-button v-if="optStep >= 0" @click="stopOptimization" variant="danger"><b-icon-x-circle-fill /></b-button>
      </b-button-group>

    </b-col>

    <b-col xl="8" lg="6">
      <h2>Settings</h2>
        <b-overlay :show="optStep >= 0">
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
                <optimization-selector class="mr-sm-2" @selected="selectOptimizationAlgorithm"></optimization-selector>
                <label class="mr-sm-2">Fixed Keys</label>
                <b-form-input v-model="optFixed" placeholder="Fixed Keys" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
              </b-form>
              <config-file :initial-content="currentOptParams" @saved="updateOptParams">
            </b-tab>

          </b-tabs>
        </b-overlay>
    </b-col>
  </b-row>

  <hr>

  <b-row>
    <b-col v-for="detail in details" xl="6">
      <layout-button :layout="detail.layout" :layout-config="selectedLayoutConfig" @remove="removeLayout"></layout-button>
      <layout-details title="Details" :layout-details="detail"></layout-details>
    </b-col>

    <b-col xl="6" v-if="details.length > 0">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
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
  data() {
    // LAYOUT_CONFIGS is defined in "vue-components.js"
    let layoutConfigs = Object.assign({}, LAYOUT_CONFIGS)
    let optParams = Object.assign({}, OPTIMIZATION_ALGORITHM_PARAMS);
    return {
      details: [],
      inputLayoutRaw: null,
      showInputValidState: false,
      wasm: null,
      rawWorker: null,
      worker: null,
      ngramProviderInitialized: false,
      evaluatorInitialized: false,
      ngramType: null,
      ngrams: null,
      corpusText: null,
      evalParamsStr: null,
      optMode: null,
      permutableKeys: null,
      optParams,
      selectedLayoutConfig: null,
      layoutConfigs,
      loading: 1,
      saOptimizationOngoing: false,
      optStep: -1,
      temperatureStr: "",
      optTotalSteps: 0,
      optFixed: ",.",
      optCancel: false,
    }
  },
  computed: {
    chartStyles() {
      return {
        height: "600px",
        position: "relative"
      }
    },

    inputLayout() {
      let layoutString = (this.inputLayoutRaw || "").replace(" ", "")
      layoutString = layoutString.toLowerCase()
      return layoutString
    },

    layoutConfig() {
      return this.layoutConfigs[this.selectedLayoutConfig]
    },

    currentOptParams() {
      return this.optParams[this.optMode]
    },

    invalidInputFeedback() {
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

    inputLayoutValid() {
      if (!this.showInputValidState) {
        return null
      } else if (this.invalidInputFeedback === null) {
        return true
      } else {
        return false
      }
    }

  },

  async created() {
    this.evalParamsStr = eval_params

    this.wasm = await import("evolve-keyboard-layout-wasm")
    this.createWorkers();

    // reduce initial value of this.loading
    this.loading -= 1
  },

  methods: {
    async createWorkers() {
      this.rawWorker = new Worker('worker.js')
      this.worker = Comlink.wrap(this.rawWorker)
      await this.worker.init()
      await this.initNgramProvider()
      await this.initLayoutEvaluator()
    },

    randomInput(fix) {
      let array = 'zluaqwbdgyjßcrieomntshvxüäöpf,.k'.split('')
      if (fix) {
        array = 'zluaqwbdgyjßcrieomntshvxüäöpfk'.split('')

      }
      for (let i = array.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [array[j], array[i]];
      }
      let res = array.join('')
      if (fix) {
        res = array.slice(0, array.length - 1).join('') + ',.' + array[array.length - 1]
      }
      this.inputLayoutRaw = res
    },
    setInput(layout) {
      this.inputLayoutRaw = layout
    },
    async evaluateInput() {
      this.showInputValidState = true
      // check if the current layout is already available in this.details
      let existing = this.details.filter((d) => d.layout == this.inputLayout)
      if (existing.length > 0) {
        this.$bvToast.toast(`Layout '${this.inputLayout}' is already available`, { variant: "primary" })
        this.showInputValidState = false
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

    async evaluateExisting() {
      if (!this.evaluatorInitialized || this.worker === null) {
        return
      }
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

    async evaluate(layout) {
      let promise = new Promise(async (resolve, reject) => {

        if (this.inputLayoutValid !== null && !this.inputLayoutValid) {
          this.$bvToast.toast("Could not evaluate Layout: " + this.invalidInputFeedback, { variant: "danger" })
          return
        }

        this.$bvToast.toast(`Evaluating layout "${layout}"`, { variant: "primary" })
        this.loading += 1
        try {
          let res = await this.worker.evaluateLayout(layout)
          res.layout = layout
          this.loading -= 1
          resolve(res)
        } catch (err) {
          this.$bvToast.toast(`Could not generate a valid layout: ${err}`, { variant: "danger" })
          this.loading -= 1
          reject(err)
        }
      })
      return promise
    },

    async initNgramProvider() {
      if (this.ngrams === null || this.worker === null) {
        return
      }
      this.loading += 1
      let data = this.ngrams
      if (this.ngramType === "from_text") {
        data = this.corpusText
      }
      try {
        await this.worker.initNgramProvider(this.ngramType, this.evalParamsStr, data)
        this.ngramProviderInitialized = true
        this.loading -= 1
      } catch (err) {
        this.$bvToast.toast(`Could not initialize ngram provider: ${err}`, { variant: "danger" })
        this.loading -= 1
        throw err
      }
    },

    async initLayoutEvaluator() {
      if (!this.ngramProviderInitialized || this.worker === null) {
        return
      }
      this.loading += 1
      try {
        await this.worker.initLayoutEvaluator(this.layoutConfig, this.evalParamsStr)
        this.permutableKeys = await this.worker.permutableKeys()
        this.evaluatorInitialized = true
        this.loading -= 1
      } catch (err) {
        this.$bvToast.toast(`Could not initialize layout evaluator: ${err}`, { variant: "danger" })
        this.loading -= 1
        throw err
      }
    },

    async updateEvalParams(evalParamsStr) {
      this.evalParamsStr = evalParamsStr

      try {
        await this.initNgramProvider()
        await this.initLayoutEvaluator()
        await this.evaluateExisting()
        this.$bvToast.toast("Saved evaluation parameters", { variant: "primary" })
      } catch (err) {
        this.$bvToast.toast("Did not update evaluation parameters", { variant: "danger" })
      }
    },

    updateOptParams(newOptParamsStr) {
      this.$bvToast.toast("Saved new optimization parameters", { variant: "primary" })
      this.optParams[this.optMode] = newOptParamsStr;
    },

    async updateNgramProviderParams(ngramType, ngramData) {
      this.ngramType = ngramType

      if (ngramType === "from_text") {
        this.corpusText = ngramData
      } else {
        this.ngrams = ngramData
      }

      try {
        await this.initNgramProvider()
        await this.initLayoutEvaluator()
        await this.evaluateExisting()
        this.$bvToast.toast("Saved ngram parameters", { variant: "primary" })
      } catch (err) {
        this.$bvToast.toast("Did not update ngram parameters", { variant: "danger" })
      }
    },

    async updateLayoutConfig(layoutConfig) {
      this.layoutConfigs[this.selectedLayoutConfig] = layoutConfig

      try {
        await this.initLayoutEvaluator()
        await this.evaluateExisting()
        this.$bvToast.toast("Saved layout configuration", { variant: "primary" })
      } catch (err) {
        this.$bvToast.toast("Did not update layout configuration", { variant: "danger" })
      }
    },

    async selectLayoutConfigType(selectedLayoutConfig) {
      this.selectedLayoutConfig = selectedLayoutConfig

      try {
        await this.initLayoutEvaluator()
        await this.evaluateExisting()
        this.$bvToast.toast(`Switched to layout configuration ${selectedLayoutConfig}`, { variant: "primary" })
      } catch (err) {
        this.$bvToast.toast(`Did not switch to layout configuration ${selectedLayoutConfig}`, { variant: "danger" })
      }
    },

    selectOptimizationAlgorithm(algorithmKey, algorithmLabel) {
      if (this.optMode !== null) {
        this.$bvToast.toast(`Switched to ${algorithmLabel}`, { variant: "primary" })
      }
      this.optMode = algorithmKey;
    },

    removeLayout(layout) {
      this.details = this.details.filter((d) => d.layout !== layout)
    },

    async startOptimization() {
      // Check if given layout_str is valid
      try {
        this.showInputValidState = true
        await this.evaluate(this.inputLayout)
        this.showInputValidState = false
      } catch (err) {
        this.optStep = -1
        this.optCancel = false
        return
      }
      this.$bvToast.toast(`Starting optimization of ${this.inputLayout}`, { variant: "primary" })

      if (this.optMode === "simulated_annealing") {
        this.saOptimization()
      } else if (this.optMode === "genevo") {
        this.genevoOtimization()
      } else {
        this.$bvToast.toast(`Error: Could not recognize mode of optimization: ${this.optMode}`, { variant: "danger" })
      }
    },
    stopOptimization() {
      if (this.optMode === "simulated_annealing") {
        this.stopSaOptimization()
      } else if (this.optMode === "genevo") {
        this.stopGenevoOtimization()
      } else {
        this.$bvToast.toast(`Error: Could not recognize mode of optimization: ${this.optMode}`, { variant: "danger" })
      }
    },

    async saOptimization() {
      this.saOptimizationOngoing = true
      await this.worker.saOptimize(
        this.inputLayout,
        this.optFixed,
        this.currentOptParams,
        Comlink.proxy(() => { }),
        Comlink.proxy(this.updateInfo),
        Comlink.proxy(this.setNewBest),
      )
      this.$bvToast.toast("Optimization finished", { variant: "primary" })
      this.evaluateInput();
      this.saOptimizationOngoing = false
    },
    updateInfo(stepNr, tStr) {
      this.optStep = stepNr
      this.temperatureStr = tStr
    },
    setNewBest(layout, cost) {
      this.$bvToast.toast(`New best layout found: ${layout}.\nCost: ${cost}`, { variant: "primary" })
      this.inputLayoutRaw = layout
    },
    stopSaOptimization() {
      this.$bvToast.toast("Stopping optimization", { variant: "primary" })
      this.rawWorker.terminate()
      this.createWorkers().then((_data) => {
        this.$bvToast.toast("Optimization finished", { variant: "primary" });
        this.optStep = -1;
        this.saOptimizationOngoing = false
      })
    },

    async genevoOtimization() {
      const optParams = await this.worker.initGenLayoutOptimizer(
        this.inputLayout,
        this.optFixed,
        this.currentOptParams
      )
      this.optTotalSteps = optParams.generation_limit
      this.optStep = 1
      this.optCancel = false

      let res
      do {
        res = await this.worker.genOptimizationStep()
        if (res !== null) {
          if (res.layout !== this.inputLayout) {
            this.$bvToast.toast(`New layout found: ${res.layout}`, { variant: "primary" })
            this.inputLayoutRaw = res.layout
          }
          this.optStep += 1
        }
      } while (res !== null && !this.optCancel)

      this.$bvToast.toast("Optimization finished", { variant: "primary" })
      this.optStep = -1
      this.optCancel = false
      this.evaluateInput();
    },
    stopGenevoOtimization() {
      this.$bvToast.toast("Stopping optimization", { variant: "primary" })
      this.optCancel = true
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
          <b-form-input v-model="publishName" :state="nameState" placeholder="Name to publish result under" class="mb-2 mr-sm-2 mb-sm-0" autofocus></b-form-input>
        </b-modal>
      </div>
    `,
  props: {
    layout: { type: String, default: "", required: true },
    layoutConfig: { type: String, required: true },
  },
  data() {
    return {
      publishName: null,
      showNameState: false,
      showModal: false,
    }
  },
  computed: {
    nameState() {
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
    remove() {
      this.$emit("remove", this.layout)
    },
    async publish(bvModalEvt) {
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
          body: JSON.stringify({ layout: this.layout, published_by: this.publishName, layout_config: this.layoutConfig })
        })
        let resData = await res.json()
        if (resData.published_by !== this.publishName) {
          this.$bvToast.toast(`Layout had already been published by "${resData.published_by}": Cost: ${resData.total_cost.toFixed(2)}`, { variant: 'warning' })
        } else {
          this.$bvToast.toast(`Successfully published layout: Cost: ${resData.total_cost.toFixed(2)}`, { variant: 'primary' })
        }
      } catch (err) {
        this.$bvToast.toast(`Error while publishing layout: ${err}`, { variant: 'danger' })
      }
    }
  },
})

Vue.component('ngram-config', {
  template: `
    <div>
      <b-form-select label="NGram Type" v-model="selected" :options="options" @change="select"></b-form-select>
      <div v-if="selected === 'from_text'">
        <b-form-textarea
          v-model="text"
          placeholder="Text to evaluate layouts on"
          rows="10"
          autofocus
        >
        </b-form-textarea>
        <b-button class="float-right" variant="primary" :disabled="text===oldText" @click="save">Analyze & Save</b-button>
      </div>
      <div v-else>
        <br>
        <span v-html="detailsHTML"></span>
      </div>
    </div>
    `,
  props: {
    defaultSelection: { type: String, default: DEFAULT_NGRAM },
  },
  data() {
    let options = []
    let description = {}
    NGRAMS.forEach(c => {
      options.push({ value: c.key, text: c.label })
      description[c.key] = c.description
    })
    options.push({ value: 'from_text', text: 'From Text' })

    return {
      selected: this.defaultSelection,
      options,
      initialLoad: true,
      oldText: "",
      text: "",
      description,
    }
  },
  created() {
    this.select()
  },
  computed: {
    detailsHTML() {
      return this.description[this.selected]
    },
  },
  methods: {
    select() {
      if (this.selected === 'from_text') {
        this.emit('from_text', this.text)
      } else {
        this.emit('prepared', this.selected)
      }
    },
    save() {
      this.oldText = this.text
      this.emit(this.selected, this.text)
    },
    emit(ngramType, ngramData) {
      if (!this.initialLoad) {
        this.$bvToast.toast("Updated ngrams", { variant: "primary" })
      } else {
        this.initialLoad = false
      }
      this.$emit('selected', ngramType, ngramData)
    }
  },
})

Vue.component('config-file', {
  template: `
    <div>
      <codemirror v-model="content" :options="options"></codemirror>
      <b-button class="float-right" variant="primary" :disabled="oldContent===content" @click="save">Save</b-button>
    </div>
    `,
  props: {
    initialContent: { type: String, default: "" },
  },
  data() {
    return {
      oldContent: this.initialContent,
      content: this.initialContent,
      options: {
        mode: 'yaml',
        lineNumbers: true,
        styleActiveLine: true,
        autoRefresh: true,
      }
    }
  },
  watch: {
    initialContent() {
      this.oldContent = this.initialContent
      this.content = this.initialContent
    },
  },
  methods: {
    save() {
      this.oldContent = this.content
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
  data() {
    return {
      plotString: null,
      layoutPlotter: null,
    }
  },
  watch: {
    layoutString() {
      this.plot()
    },
    wasm() {
      this.update()
    },
    layoutConfig() {
      this.update()
    },
    permutableKeys() {
      this.update()
    },
  },
  mounted() {
    this.update()
  },
  methods: {
    update() {
      if (this.wasm === null || this.layoutConfig === null || this.permutableKeys === null) return
      try {
        this.layoutPlotter = this.wasm.LayoutPlotter.new(this.layoutConfig)
      } catch (err) {
        this.$bvToast.toast(`Error plotting the layout: ${err}`, { variant: 'danger' })
      }
      this.plot()
    },
    plot() {
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
