<script>
export default {
  name: "console"
};
</script>

<style lang="scss" global>
.console {
  width: 100%;
  border-left: 1px solid $GRAY;
  padding-left: 16px;
  height: calc(100vh - 48px);
  overflow-y: auto;
}

.success,
.error {
  &:last-of-type {
    padding-bottom: 40px;
  }
}

.success {
  color: $WHITE;
}

.error {
  color: $RED;
}
</style>

<template>
  <div class="console" id="console" />
</template>
