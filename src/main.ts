import Vue from "vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import store from "./store";
import "./utils";

Vue.config.productionTip = false;

new Vue({
  router,
  store,
  el: "#app",
  render: (h) => h(App),
});
