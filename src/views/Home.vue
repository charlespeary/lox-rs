<script>
import PrismEditor from "vue-prism-editor";
import "prismjs/themes/prism-twilight.css";
import Buttons from "@/components/Buttons";
import Console from "@/components/Console";

export default {
  name: "home",
  components: {
    Buttons,
    PrismEditor,
    Console
  },
  computed: {
    code: {
      set: function(code) {
        this.$store.commit("updateCode", code);
      },
      get: function() {
        return this.$store.state.code;
      }
    }
  }
};
</script>

<style lang="scss">
body {
  overflow: hidden;
}

.home {
  &__container {
    background: $BLACK;
    display: flex;
    flex-wrap: nowrap;
    padding: 16px;
  }
}

.editor {
  display: flex;
  flex-wrap: nowrap;
  width: 100%;
  padding-bottom: 80px;
  height: calc(100vh - 48px);
  overflow-y: auto;
  border: none;

  .prism-editor__line-numbers {
    background: $BLACK !important;
    width: 32px;
    padding: 0 !important;
    margin: 0 !important;
  }

  pre {
    box-shadow: none;
    background: $BLACK;
    outline: none;
    margin: 0;
    width: 100%;
    border: none;
    padding: 0 0 32px 0;
    overflow: hidden;
    height: 100%;
    display: table;
  }
}
</style>

<template>
  <div class="home">
    <buttons />
    <div class="home__container">
      <PrismEditor lineNumbers v-model="code" language="js" class="editor" />
      <Console />
    </div>
  </div>
</template>
