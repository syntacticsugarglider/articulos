<template lang="pug">
  .page(@click="focus")
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
      if (this.content.length === 1) {
        return;
      }
      this.content.splice(index, 1);
      this.$nextTick(() => {
        const el = (this.$el.querySelectorAll('.content')[Math.max(index - 1, 0)] as HTMLElement);
        setTimeout(() => {
          el.focus();
          if (el.childNodes.length > 0) {
            console.log((el.parentElement as any).__vue__);
            (el as any).parentElement.__vue__.focusAtEnd();
          }
        }, 0);
      });
    },
    focus(e: MouseEvent) {
      const sel = document.getSelection()!;
      if ((e.target as Node).isSameNode(this.$el)) {
        if (!document.activeElement!.matches('.content')) {
          const contents = this.$el.querySelectorAll('.content');
          const lastSection = contents[contents.length - 1];
          (lastSection as any).parentElement.__vue__.focusAtEnd();
          (lastSection as HTMLElement).focus();
        }
      }
    },
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