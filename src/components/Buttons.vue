<script>
import { mapActions } from "vuex";
import examples from "../examples";

export default {
  name: "buttons",
  methods: {
    ...mapActions(["runCode"]),
    clearConsole: function() {
      const container = document.querySelector("#console");
      container.textContent = "";
    },
    setExample: function(example) {
      this.$store.commit("updateCode", examples[example]);
    },
    clearCode: function() {
      this.$store.commit("updateCode", " ");
    },
  },
};
</script>

<style lang="scss" scoped>
.buttons {
  display: flex;
  justify-content: center;
  background: $LIGHT_BLACK;

  button {
    height: 48px;
    width: 96px;
    outline: none;
    border: none;
    background-color: $LIGHT_BLACK_2;
    color: $WHITE;
    transition: opacity 0.3s linear;
    margin: 0 8px;

    &:hover {
      cursor: pointer;
      opacity: 0.8;
    }
  }
}
</style>

<template>
  <div class="buttons">
    <button v-on:click="clearCode">
      CLEAR EDITOR
    </button>
    <button v-on:click="setExample('languageOverview')">
      LANGUAGE OVERVIEW
    </button>
    <button v-on:click="setExample('lexerErrors')">
      LEXER ERRORS
    </button>
    <button v-on:click="setExample('parserErrors')">
      PARSER ERRORS
    </button>
    <button v-on:click="setExample('staticAnalysisErrors')">
      STATIC ANALYSIS
    </button>
    <button v-on:click="runCode">RUN CODE</button>
    <button v-on:click="clearConsole">CLEAR CONSOLE</button>
  </div>
</template>
