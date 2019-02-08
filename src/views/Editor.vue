<template lang="pug">
  .page
    EditableContent(v-for="entry in content.entries()", @new="createNew", :key="entry[0]")
</template>

<script lang="ts">
import Vue from 'vue';

import EditableContent from '@/components/EditableContent.vue';

export default Vue.extend({
  props: [],
  components: {EditableContent},
  data() {
    return {
      content: [''],
    };
  },
  methods: {
    createNew() {
      this.content.push('');
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