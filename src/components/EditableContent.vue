<template lang="pug">
  .container(@keydown="keydown")
    p.content(v-if="type === 1", ref="input", contenteditable=true, @input="update") {{ content }}
    h1.content(v-else-if="type === 2", ref="input", contenteditable=true, @input="update") {{ content }}
    h2.content(v-else-if="type === 3", ref="input", contenteditable=true, @input="update") {{ content }}
</template>

<script lang="ts">
interface InputEvent {
  inputType: string;
}

enum Type {
  Paragraph = 1,
  Header,
  Header2,
}

import Vue from 'vue';

export default Vue.extend({
  props: [],
  data() {
    return {
      content: '',
      type: Type.Paragraph,
    };
  },
  mounted() {
    (this.$refs.input as HTMLElement).focus();
  },
  methods: {
    keydown(e: KeyboardEvent) {
      const sel = document.getSelection()!;
      if (sel.anchorOffset === 0 && e.keyCode === 8 && sel.isCollapsed)  {
        this.setType(Type.Paragraph);
      }
    },
    update(e: InputEvent) {
      this.content = (this.$refs.input as HTMLElement).innerHTML;
      this.content = this.content.replace('&nbsp;', ' ');
      if (this.content.slice(0, 2) === '# ') {
        this.content = this.content.slice(2);
        this.setType(Type.Header);
      } else if (this.content.slice(0, 3) === '## ') {
        this.content = this.content.slice(3);
        this.setType(Type.Header2);
      } else if (/.*(<br\\?>){3,}$/.test(this.content)) {
        this.content = this.content.substr(0, /(<br\\?>){3,}$/.exec(this.content)!.index);
        (this.$refs.input as HTMLElement).innerHTML = this.content;
        this.$emit('new');
      }
    },
    setType(t: Type) {
      let offset = document.getSelection()!.anchorOffset;
      if (t === Type.Paragraph) {
        if (this.type === Type.Header) {
          this.content = '#' + this.content;
          offset += 1;
        }
      }
      this.type = t;
      this.$nextTick(() => {
          (this.$refs.input as HTMLElement).focus();
      });
    },
  },
});
</script>

<style scoped lang="sass">
@keyframes fade
  from
    opacity: 0
  to
    opacity: 0.75
.content
  outline: none
  caret-color: var(--secondary-accent-color)
  min-width: 0
  display: inline-block
  position: relative
  width: 100%
.content::before
  display: block
  color: var(--secondary-accent-color)
  position: absolute
  left: -1em
  animation: fade 0.15s ease
  transition: opacity 0.15s ease
  font-weight: bold
  opacity: 0.5
.content:focus::before
  opacity: 0.75
h1.content::before
  content: "#"
h2.content::before
  content: "##"
  left: -2em
p.content::before
  content: "¶"
  margin-top: -1.5px
</style>