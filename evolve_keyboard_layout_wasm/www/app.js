import * as wasm from "evolve-keyboard-layout-wasm"

import unigrams from '../../1-gramme.arne.no-special.txt'
import bigrams from '../../2-gramme.arne.no-special.txt'
import trigrams from '../../3-gramme.arne.no-special.txt'

import layout_config from '../../config/standard_keyboard.yml'
import eval_params from '../../config/evaluation_parameters.yml'


Vue.component('evaluator-app', {
    template: `
<b-container>

  <b-row>
    <b-col xl="6">
      <b-form inline>
        <b-form-input v-model="layout" debounce="500" placeholder="Layout" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        <b-button @click="evaluate">Evaluate</b-button>
      </b-form>
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
        this.layoutEvaluator = wasm.LayoutEvaluator.new(layout_config, eval_params, unigrams, bigrams, trigrams)
    },
    mounted () {
    },
    methods: {
        evaluate () {
            if (this.layout === null) return
            let details = this.layoutEvaluator.evaluate(this.layout)
            details.layout = this.layout
            this.details = details
        }
    }
})

var app = new Vue({
    el: '#app',
})
