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

declare var InstallTrigger: any;

export default Vue.extend({
  props: ['value'],
  data() {
    return {
      content: this.$props.value.content,
      type: this.$props.value.type,
      isFirefox: typeof InstallTrigger !== 'undefined',
    };
  },
  mounted() {
    const el = this.$refs.input as HTMLElement;
    el.focus();
    const val = el.innerHTML;
    el.childNodes.forEach((c) =>
      c.remove(),
    );
    el.innerHTML = this.isFirefox && val === '' ? `<br type="_moz">` : val;
    el.focus();
  },
  methods: {
    keydown(e: KeyboardEvent) {
      const sel = document.getSelection()!;
      if (sel.anchorOffset === 0 && e.keyCode === 8 && sel.isCollapsed)  {
        if (this.type !== Type.Paragraph) {
          this.setType(Type.Paragraph);
        } else {
          this.$emit('destroy');
        }
      } else if (e.keyCode === 38) {
        if (this.$el.previousSibling) {
          setTimeout(() => {
            const el = (this.$el.previousSibling! as HTMLElement).firstChild!;
            if (el.childNodes.length > 0) {
              const lastNode = el.childNodes[el.childNodes.length - 1];
              sel.collapse(lastNode, lastNode.nodeValue!.length);
            }
            ((this.$el.previousSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 37) {
        if (this.$el.previousSibling && sel.anchorOffset === 0 && sel.isCollapsed) {
          setTimeout(() => {
            const el = (this.$el.previousSibling! as HTMLElement).firstChild!;
            if (el.childNodes.length > 0) {
              const lastNode = el.childNodes[el.childNodes.length - 1];
              sel.collapse(lastNode, lastNode.nodeValue!.length);
            }
            ((this.$el.previousSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 39) {
        if (this.$el.nextSibling && sel.anchorOffset === this.content.length && sel.isCollapsed) {
          setTimeout(() => {
            const el = (this.$el.nextSibling! as HTMLElement);
            sel.collapse(el.childNodes[0], 0);
            ((this.$el.nextSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 40) {
        if (this.$el.nextSibling) {
          ((this.$el.nextSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
        }
      }
      (this.$refs.input as Node).normalize();
    },
    update(e: InputEvent) {
      this.content = (this.$refs.input as HTMLElement).innerHTML;
      this.content = this.content
        .replace('&nbsp;', ' ')
        .replace(/^\s*/, '')
        .replace(/<br\\?>(?!<br\\?>)$/, '');
      if (this.content === '' && this.isFirefox) {
        this.content = `<br type="_moz">`;
      }
      if (this.content.slice(0, 2) === '# ' && this.type === Type.Paragraph) {
        this.content = this.content.slice(2);
        this.setType(Type.Header);
      } else if (this.content.slice(0, 3) === '## ' && this.type === Type.Paragraph) {
        this.content = this.content.slice(3);
        this.setType(Type.Header2);
      } else if (/.*(<br\\?>)$/.test(this.content)) {
        this.content = this.content.substr(0, /(<br\\?>)*$/.exec(this.content)!.index);
        (this.$refs.input as HTMLElement).innerHTML = this.content;
        this.$emit('new');
      } else if (/.*<br\\?>.*/.test(this.content)) {
        const lines = this.content.split(/<br\\?>/);
        this.content = lines[0];
        (this.$refs.input as HTMLElement).innerHTML = this.content;
        this.$emit('new', {content: lines[1], type: this.type});
      }
      (this.$refs.input as Node).normalize();
      this.$emit('input', {content: this.content, type: this.type});
    },
    setType(t: Type) {
      let offset = 0;
      if (t === Type.Paragraph) {
        if (this.type === Type.Header) {
          this.content = '#' + this.content;
          offset = 1;
        } else if (this.type === Type.Header2) {
          this.content = '##' + this.content;
          offset = 2;
        }
      }
      this.type = t;
      setTimeout(() => {
        const el = this.$refs.input as HTMLElement;
        el.focus();
        setTimeout(() => {
          const val = el.innerHTML;
          el.childNodes.forEach((c) =>
            c.remove(),
          );
          el.innerHTML = val;
          el.focus();
          const sel = window.getSelection();
          sel.collapse(el.childNodes[0], offset);
        }, 0);
      }, 0);
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