<template lang="pug">
  .page
    EditableContent(v-for="entry in content.entries()", @new="(content) => {createNew(entry[0], content)}", @destroy="destroy(entry[0])", :key="entry[1][1]", v-model="entry[1][0]")
</template>

<script lang="ts">
import Vue from 'vue';

import EditableContent from '@/components/EditableContent.vue';

interface Section {
  content: string;
  type: number;
}

export default Vue.extend({
  props: [],
  components: {EditableContent},
  data() {
    return {
      content: [[{content: '', type: 1}, 1]],
      accu: 1,
    };
  },
  methods: {
    createNew(index: number, content: Section) {
      this.accu++;
      this.content.splice(index + 1, 0, [content || {content: '', type: 1}, this.accu]);
    },
    destroy(index: number) {
      if (index === 0) {
        return;
      }
      this.content.splice(index, 1);
      const el = (this.$el.querySelectorAll('.content')[index - 1] as HTMLElement);
      setTimeout(() => {
        el.focus();
        if (el.childNodes.length > 0) {
          const sel = window.getSelection();
          const lastNode = el.childNodes[el.childNodes.length - 1];
          sel.collapse(lastNode, lastNode.nodeValue!.length);
        }
      }, 0);
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