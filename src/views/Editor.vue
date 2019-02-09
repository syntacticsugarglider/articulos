<template lang="pug">
  .page
    EditableContent(v-for="entry in content.entries()", @new="createNew(entry[0])", :key="entry[1][1]")
</template>

<script lang="ts">
import Vue from 'vue';

import EditableContent from '@/components/EditableContent.vue';

export default Vue.extend({
  props: [],
  components: {EditableContent},
  data() {
    return {
      content: [['', 1]],
      accu: 1,
    };
  },
  methods: {
    createNew(index: number) {
      this.accu++;
      this.content.splice(index + 1, 0, ['', this.accu]);
    },
  },
});
</script>

<style scoped lang="sass">
.page
  text-align: initial
  max-width: 700px
  padding: 30px
  width: calc(100vw - 100px)
  margin: auto
  border-radius: 10px
  margin-top: 50px
</style>