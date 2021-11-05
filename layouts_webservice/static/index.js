Vue.component('layouts-app', {
    template: `
<b-container-fluid>
  <b-row>
    <b-col cols="4">
      <layouts-table :url="url" @details="setDetails"></layouts-table>
    </b-col>

    <b-col cols="4">
        <layout-details title="Details 1" :base-url="url" :layout="layout_1">
    </b-col>
    <b-col cols="4">
        <layout-details title="Details 2" :base-url="url" :layout="layout_2">
    </b-col>
  </b-row>

</b-container-fluid>
`,
    props: {
        url: { type: String, default: "http://localhost:8000/api" }
    },
    data () {
        return {
            layout_1: null,
            layout_2: null
        }
    },
    computed: {
    },
    created () {
    },
    methods: {
        setDetails (target, layout) {
            if (target == 1) this.layout_1 = layout;
            if (target == 2) this.layout_2 = layout;
        }
    }
})


Vue.component('layout-details', {
    template: `
<b-jumbotron :header="headline" :lead="leadline" header-level="5">
    <div v-if="layoutDetails !== null">
        <pre v-html="plot"></pre>
        <h2>Gesamtkosten: {{ totalCost }}</h2>
        <hr>
        <pre v-html="printed"></pre>
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
<b-table small head-variant="light" sort-by="total_cost" :items="rows" :fields="fields" :tbody-tr-class="rowClass">
  <template #cell(to_details)="data">
<b-button-group>
    <b-button size="sm" variant="light" @click="onClick(1, data.item.layout)">1</b-button>
    <b-button size="sm" variant="light "@click="onClick(2, data.item.layout)">2</b-button>
</b-button-groub>
  </template>
</b-table>`,
    props: {
        'url': {type: String, default: null},
    },
    data () {
        return {
            layouts: []
        }

    },
    computed: {
        rows () {
            const res = this.layouts.map((layout, i) => {
                const row = {
                    layout: layout.layout,
                    total_cost: layout.total_cost,
                    published_by: layout.published_by,
                    highlight: layout.highlight
                }
                return row
            })
            return res
        },
        fields () {
            return [
                {
                    key: 'layout',
                    label: 'Layout',
                },
                {
                    key: 'total_cost',
                    label: 'Kosten',
                    sortable: true,
                    formatter: (c) => c.toFixed(2)
                },
                {
                    key: 'published_by',
                    label: 'Veröffentlicht von',
                    sortable: true
                },
                {
                    key: 'highlight',
                    label: 'Bekannt',
                    sortable: true
                },
                {
                    key: 'to_details',
                    label: 'Ansehen'
                }
            ]
        },
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
            if (item.highlight) return 'table-primary'
        },
        onClick (target, layout) {
            this.$emit("details", target, layout)
        }
    }
})

var app = new Vue({
    el: '#app',
})
