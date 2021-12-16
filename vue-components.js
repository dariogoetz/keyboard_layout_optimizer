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
        layoutDetails: { type: Array, default: [] },
        relative: { type: Boolean, default: true },
        logscale: { type: Boolean, default: false }
    },
    data () {
        return {
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

                let label = details.layout
                if (details.published_by) {
                    label = `${details.published_by} (${details.layout})`
                }

                datasets.push({
                    label,
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
        layoutDetails () {
            this.render()
        },
        relative () {
            this.render()
        },
        logscale () {
            this.render()
        },
        chartData () {
            this.render()
        }
    },
    methods: {
        render () {
            this.renderChart(this.chartData, this.options)
        }
    }
})


Vue.component('layout-details', {
    template: `
<b-jumbotron :header="headline" :lead="leadline" header-level="5">
    <div v-if="layoutDetails !== null">
        <pre style="overflow-y: hidden"><code v-html="plot"></code></pre>
        <h2>Gesamtkosten: {{ totalCost }}</h2>
        <hr>
        <pre style="overflow-y: hidden"><code v-html="printed"></code></pre>
    </div>
</b-jumbotron>
`,
    props: {
        layoutDetails: { type: Object, default: null },
        title: { type: String, default: null },
    },
    data () {
        return {
        }
    },
    watch: {
        layoutDetails () {},
    },
    computed: {
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
            if (!this.layoutDetails.published_by) return this.layoutDetails.layout
            return `${ this.layoutDetails.layout } (${ this.layoutDetails.published_by })`
        },
    },
})


Vue.component('layouts-table', {
    template: `
<div>
  <b-table
    sticky-header="600px"
    primary-key="layout"
    small
    head-variant="light"
    sort-by="total_cost"
    sort-icon-left
    no-sort-reset
    :items="rows"
    :fields="fields"
    :filter="filter"
    :per-page="perPage"
    :current-page="currentPage"
    :tbody-tr-class="rowClass"
    @row-clicked="onRowClicked"
    @filtered="onFiltered"
   >
  </b-table>
  <b-pagination
    v-if="filteredRows > perPage"
    v-model="currentPage"
    :total-rows="filteredRows"
    :per-page="perPage"
    size="sm"
  >
  </b-pagination>
</div>
    `,
    props: {
        url: { type: String, default: null },
        bestInFamily: { type: Boolean, default: true },
        filter: { type: String, default: null },
        perPage: { type: Number, default: 500 },
    },
    data () {
        return {
            layouts: [],
            currentPage: 1,
            filteredRows: 0,
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
                }
                return row
            })
            return res
        },
        fields () {
            return [
                {
                    key: 'published_by',
                    label: 'Published by',
                    sortable: true
                },
                {
                    key: 'total_cost',
                    label: 'Cost',
                    sortable: true,
                    formatter: (c) => c.toFixed(2)
                },
                {
                    key: 'layout',
                    label: 'Layout',
                },
                {
                    key: 'family',
                    label: 'Family',
                    sortable: true
                },
                {
                    key: 'periodComma',
                    label: 'Period/Comma',
                    sortable: true,
                },
                {
                    key: 'highlight',
                    label: 'Well-known',
                    sortable: true
                }
            ]
        },
    },
    created () {
        this.fetchLayouts()
    },
    watch: {
        bestInFamily () {
            this.rows.forEach(item => item.selected = false)
            this.$emit('details', [])
        },
    },
    methods: {
        fetchLayouts () {
            if (this.url === null) return null
            return fetch(this.url)
                .then(response => response.json())
                .then(data => {
                    this.layouts = data
                    this.filteredRows = data.length
                })
        },
        rowClass (item, type) {
            if (!item || type !== 'row') return
            if (item.selected) return 'table-secondary'
            if (item.highlight) return 'table-primary'
        },
        onRowClicked (item) {
            if (item.selected) {
                this.$set(item, 'selected', false)
            } else {
                this.$set(item, 'selected', true)
            }
            let selection = this.rows.filter(item => item.selected).sort((a, b) => a.total_cost - b.total_cost)
            this.$emit("details", selection)
        },
        onFiltered (items) {
            this.filteredRows = items.length
            this.currentPage = 1
        },
    }
})

