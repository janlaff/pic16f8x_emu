import Vue from 'vue'
import vuetify from "./plugins/vuetify"
import App from './App.vue'

// Fontawesome setup
import { library } from "@fortawesome/fontawesome-svg-core"
import { faPlay, faStop, faFolder } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome"

library.add({
  faPlay,
  faStop,
  faFolder
})

Vue.component('font-awesome-icon', FontAwesomeIcon)

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  vuetify
}).$mount('#app')
