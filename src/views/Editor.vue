<template lang="pug">
  .page
    EditableContent
</template>

<script lang="ts">
import Vue from 'vue';

import EditableContent from '@/components/EditableContent.vue';

interface Section {
  content: string;
  type: number;
}

export default Vue.extend({
  components: {EditableContent},
  data() {
    return {};
  },
});
</script>

<style scoped lang="sass">
.page
  text-align: initial
  max-width: 700px
  padding: 30px
  padding-top: 80px
  padding-bottom: 80px
  width: calc(100vw - 100px)
  margin: auto
  border-radius: 10px
  min-height: 100vh
</style>