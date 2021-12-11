Vue.component('layouts-app', {
    template: `
<b-container fluid>

  <b-row>
    <b-col xl="6">
      <b-form inline>
        <b-form-input v-model="filter" debounce="500" placeholder="Filter" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        <b-form-checkbox v-model="bestInFamily" class="mb-2 mr-sm-2 mb-sm-0">only show best in family</b-form-checkbox>
      </b-form>
      <layouts-table :url="url" :bestInFamily="bestInFamily" :filter="filter" @details="setDetails"></layouts-table>
    </b-col>

    <b-col xl="6">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
        <b-form-checkbox v-if="!relative" v-model="logscale" inline>logarithmic scale</b-form-checkbox>
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
                return fetch(`${this.url}/${layoutData.layout}`)
                    .then(response => response.json())
            })

            Promise.all(res)
                .then(layoutDetails => {
                    this.details = layoutDetails
                })
        },
    }
})

var app = new Vue({
    el: '#app',
})
