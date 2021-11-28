Vue.component('layouts-app', {
    template: `
<b-container fluid>

  <b-row>
    <b-col xl="6">
      <b-form inline>
        <b-form-input v-model="filter" placeholder="Filter" class="mb-2 mr-sm-2 mb-sm-0"></b-form-input>
        <b-form-checkbox v-model="bestInFamily" class="mb-2 mr-sm-2 mb-sm-0">only show best in family</b-form-checkbox>
      </b-form>
      <layouts-table :url="url" :bestInFamily="bestInFamily" :filter="filter" @details="setDetails"></layouts-table>
    </b-col>

    <b-col xl="6">
      <b-form inline>
        <b-form-checkbox v-model="relative"inline>relative barplot</b-form-checkbox>
        <b-form-checkbox v-if="!relative" v-model="logscale" inline>logarithmic scale</b-form-checkbox>
      </b-form>
      <layout-barplot :base-url="url" :layout-data="details" :relative="relative" :logscale="logscale && !relative" :styles="chartStyles"></layout-barplot>
    </b-col>
  </b-row>

  <b-row>
    <b-col cols="6" v-for="detail in details">
      <layout-details title="Details" :base-url="url" :layout="detail.layout"></layout-details>
    </b-col>
  </b-row>

</b-container-fluid>
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
            this.details = items
        },
    }
})

const COLORS = [
  '#4dc9f6',
  '#f67019',
  '#f53794',
  '#537bc4',
  '#acc236',
  '#166a8f',
  '#00a950',
  '#58595b',
  '#8549ba'
];

Vue.component('layout-barplot', {
    extends: VueChartJs.Bar,
    props: {
        layoutData: { type: Array, default: [] },
        baseUrl: { type: String, default: null },
        relative: { type: Boolean, default: true },
        logscale: { type: Boolean, default: false }
    },
    data () {
        return {
            layoutDetails: [],
        }
    },
    computed: {
        chartData () {
            const datasets = []
            const n_datasets = this.layoutDetails.length
            let labels = ["Total"]
            if (!this.relative || n_datasets === 1) {
                labels[0] = [`Total / 10`]
            }
            // totals is used for relative values
            const totals = []
            this.layoutDetails.forEach((details, i) => {
                // divide total cost by 10 for scaling reasons
                let total = details.total_cost
                if (!this.relative || n_datasets === 1) {
                    total = total / 10
                }

                // the first metric shown will be the total cost
                let values = [total]
                if (i === 0) {
                    totals.push(total)
                } else {
                    totals[0] += total
                }

                // collect weighted metric costs from metric results datastructure
                let j = 1
                details.details.individual_results.forEach(metricTypeResults => {
                    metricTypeResults.metric_costs.forEach(mc => {
                        if (i === 0) {
                            labels.push(mc.core.name)
                            totals.push(mc.weighted_cost)
                        } else {
                            totals[j] += mc.weighted_cost
                            j += 1
                        }
                        values.push(mc.weighted_cost)
                    })
                })
                datasets.push({
                    label: `${details.published_by} (${details.layout})`,
                    backgroundColor: COLORS[datasets.length],
                    data: values
                })
            })

            // if relative numbers are to be shown (and more than one dataset is to be compared), subtract means
            if (this.relative && n_datasets > 1) {
                for (dataset of datasets) {
                    dataset.data = dataset.data.map((v, j) => {
                        return v - (totals[j] / n_datasets)
                    })
                }
            }
            return {
                labels: labels,
                datasets: datasets
            }
        },
        options () {
            const options = {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    xAxes: [{
                        ticks: {
                            autoSkip: false,
                            maxRotation: 90,
                            minRotation: 90
                        }
                    }],
                    yAxes: [{
                        type: this.logscale ? "logarithmic" : "linear",
                    }]
                }
            }

            return options
        },
    },
    mounted () {
        this.renderChart(this.chartData, this.options)
    },
    watch: {
        relative () {
            this.render()
        },
        logscale () {
            this.render()
        },
        layoutData () {
            this.fetch()
        },
        chartData () {
            this.render()
        }
    },
    methods: {
        fetch () {
            const res = this.layoutData.map(layoutData => {
                const url = this.url(layoutData.layout)
                if (url === null) return null
                return fetch(url)
                    .then(response => response.json())
            })

            Promise.all(res)
                .then(data => {
                    this.layoutDetails = data
                })

        },
        url (layout) {
            if (this.baseUrl === null || layout === null) return null
            return `${this.baseUrl}/${layout}`
        },
        render () {
            this.renderChart(this.chartData, this.options)
        }
    }
})


Vue.component('layout-details', {
    template: `
<b-jumbotron :header="headline" :lead="leadline" header-level="5">
    <div v-if="layoutDetails !== null">
        <pre><code v-html="plot"></code></pre>
        <h2>Gesamtkosten: {{ totalCost }}</h2>
        <hr>
        <pre><code v-html="printed"></code></pre>
    </div>
</b-jumbotron>
`,
    props: {
        layout: { type: String, default: null },
        title: { type: String, default: null },
        baseUrl: { type: String, default: null },
    },
    data () {
        return {
            layoutDetails: null,
        }
    },
    watch: {
        layout () {
            this.fetch()
        }
    },
    created () {
        this.fetch()
    },
    computed: {
        url () {
            if (this.baseUrl === null || this.layout === null) return null
            return `${this.baseUrl}/${this.layout}`
        },
        plot () {
            if (this.layoutDetails === null) return ""
            const p = this.layoutDetails.plot.replaceAll("\n", "<br>")
            return p
        },
        printed () {
            if (this.layoutDetails === null) return ""
            const p = this.layoutDetails.printed.replaceAll("\n", "<br>")
            return p
        },
        totalCost () {
            if (this.layoutDetails === null) return ""
            return this.layoutDetails.total_cost.toFixed(2)
        },
        headline () {
            if (this.title === null) return ""
            return this.title
        },
        leadline () {
            if (this.layoutDetails === null) return ""
            return `${ this.layout } (${ this.layoutDetails.published_by })`
        },
    },
    methods: {
        fetch () {
            if (this.url === null) return null
            return fetch(this.url)
                .then(response => response.json())
                .then(data => this.layoutDetails = data)
        },
    }
})


Vue.component('layouts-table', {
    template: `
<b-table
  sticky-header="600px"
  small
  head-variant="light"
  sort-by="total_cost"
  sort-icon-left
  no-sort-reset
  :items="rows"
  :fields="fields"
  :filter="filter"
  :tbody-tr-class="rowClass"
  @row-clicked="onRowClicked"
>
</b-table>`,
    props: {
        url: { type: String, default: null },
        bestInFamily: { type: Boolean, default: true },
        filter: { type: String, default: null }
    },
    data () {
        return {
            layouts: []
        }

    },
    computed: {
        rows () {
            let layouts = this.layouts
            if (this.bestInFamily) {
                let familyMap = new Map()
                this.layouts.forEach(layout => {
                    let family = layout.layout.slice(12, 22)
                    let familyBest = familyMap.get(family)
                    if (familyBest === undefined || layout.total_cost < familyBest.total_cost) {
                        familyMap.set(family, layout)
                    }
                })
                layouts = Array.from(familyMap, ([k, v]) => v)
            }

            const res = layouts.map((layout, i) => {
                const row = {
                    layout: layout.layout,
                    total_cost: layout.total_cost,
                    published_by: layout.published_by,
                    highlight: layout.highlight,
                    family: layout.layout.slice(12, 22),
                    periodComma: layout.layout.slice(29, 31) == ',.' ? 'standard' : 'unusual',
                    selected: false,
                }
                return row
            })
            return res
        },
        fields () {
            return [
                {
                    key: 'published_by',
                    label: 'Veröffentlicht von',
                    sortable: true
                },
                {
                    key: 'total_cost',
                    label: 'Kosten',
                    sortable: true,
                    formatter: (c) => c.toFixed(2)
                },
                {
                    key: 'layout',
                    label: 'Layout',
                },
                {
                    key: 'family',
                    label: 'Familie',
                    sortable: true
                },
                {
                    key: 'periodComma',
                    label: 'Punkt/Komma',
                    sortable: true,
                },
                {
                    key: 'highlight',
                    label: 'Bekannt',
                    sortable: true
                }
            ]
        }
    },
    created () {
        this.fetchLayouts()
    },
    methods: {
        fetchLayouts () {
            if (this.url === null) return null
            return fetch(this.url)
                .then(response => response.json())
                .then(data => { this.layouts = data })
        },
        rowClass (item, type) {
            if (!item || type !== 'row') return
            if (item.selected) return 'table-secondary'
            if (item.highlight) return 'table-primary'
        },
        onRowClicked(item) {
            if(item.selected){
                this.$set(item, 'selected', false)
            } else {
                this.$set(item, 'selected', true)
            }
            this.$emit("details", this.selectedRows())
        },
        selectedRows() {
            const res = this.rows.filter(item => item.selected)
            res.sort((a, b) => a.total_cost - b.total_cost)
            return res
        }
    }
})

var app = new Vue({
    el: '#app',
})
