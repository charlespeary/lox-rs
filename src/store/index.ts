import { runCode } from "@/utils";
import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

type State = {
  code: string;
};

export default new Vuex.Store({
  state: {
    code: "\n\n\n\n\n\n\n\n\n\n"
  } as State,
  mutations: {
    updateCode(state, code: string) {
      state.code = code;
    }
  },
  actions: {
    runCode(action) {
      const { state } = action;
      const { code } = state;
      console.log("Running code");
      runCode(code);
    }
  },
  modules: {}
});
