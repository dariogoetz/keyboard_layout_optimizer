Vue.component('layouts-app', {
    template: `
<b-container fluid>

  <h1>Evaluate Keyboard Layouts</h1>
  Evaluate your own layouts and inspect configuration parameters at <a href="https://dariogoetz.github.io/keyboard_layout_optimizer/">https://dariogoetz.github.io/keyboard_layout_optimizer/</a>

  <hr>

  <b-row>
    <b-col xl="6">
      <b-form inline @submit.stop.prevent>
        <keyboard-selector @selected="keyboardSelected"></keyboard-selector>
        <b-form-input v-model="filter" debounce="500" placeholder="Filter" class="mb-2 mr-sm-2 mb-sm-0 ml-sm-2"></b-form-input>
        <b-form-checkbox v-model="bestInFamily" class="mb-2 mr-sm-2 mb-sm-0">only show best in family (clears selection)</b-form-checkbox>
      </b-form>
      <layouts-table :url="url" :layout-config="layoutConfig" :bestInFamily="bestInFamily" :filter="filter" @details="setDetails"></layouts-table>
    </b-col>

    <b-col xl="6">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
      </b-form>
      <layout-barplot :layout-details="details" :relative="relative" :logscale="logscale && !relative" :styles="chartStyles"></layout-barplot>
    </b-col>
  </b-row>

  <b-row>
    <b-col cols="6" v-for="detail in details">
      <layout-details title="Details" :layout-details="detail"></layout-details>
    </b-col>
  </b-row>

</b-container>
`,
    props: {
        url: { type: String, default: "/api" },
        relative: { type: Boolean, default: false },
        logscale: { type: Boolean, default: false },
        bestInFamily: { type: Boolean, default: false }
    },
    data () {
        return {
            details: [],
            filter: null,
            layoutConfig: DEFAULT_LAYOUT_CONFIG,
        }
    },
    computed: {
        chartStyles () {
            return {
                height: "600px",
                position: "relative"
            }
        },
    },
    created () {
    },
    mounted () {
    },
    methods: {
        setDetails (items) {
            // fetch details for all selected items
            const res = items.map(layoutData => {
                if (this.url === null) return null
                return fetch(`${this.url}/${layoutData.layout.replace("/", "__")}?layout_config=${this.layoutConfig}`)
                    .then(response => response.json())
            })

            Promise.all(res)
                .then(layoutDetails => {
                    this.details = layoutDetails
                })
        },
        keyboardSelected (layoutConfig) {
            this.details = []
            this.filter = null
            this.layoutConfig = layoutConfig
        },
    }
})

var app = new Vue({
    el: '#app',
})
