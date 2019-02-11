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
      if (this.type === Type.Paragraph) {
        if (e.key === '*') {
          e.preventDefault();
          return;
        }
      }
      const sel = document.getSelection()!;
      if (e.keyCode === 8 && this.isFocusedAtStart())  {
        e.preventDefault();
        if (this.type !== Type.Paragraph) {
          this.setType(Type.Paragraph);
        } else {
          this.$emit('destroy');
        }
      } else if (e.keyCode === 38) {
        if (this.$el.previousSibling && this.isFocusedAtFirstNode()) {
          e.preventDefault();
          setTimeout(() => {
            const el = (this.$el.previousSibling! as any);
            if (el.firstChild.childNodes.length > 0) {
              el.__vue__.focusAtEnd();
            }
            ((this.$el.previousSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 37) {
        if (this.$el.previousSibling && this.isFocusedAtStart()) {
          e.preventDefault();
          setTimeout(() => {
            const el = (this.$el.previousSibling! as any)!;
            if (el.firstChild.childNodes.length > 0) {
              el.__vue__.focusAtEnd();
            }
            ((this.$el.previousSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 39) {
        if (this.$el.nextSibling && this.isFocusedAtEnd()) {
          e.preventDefault();
          setTimeout(() => {
            const el = (this.$el.nextSibling! as HTMLElement);
            sel.collapse(el.childNodes[0], 0);
            ((this.$el.nextSibling! as HTMLElement).querySelector('.content') as HTMLElement)!.focus();
          }, 0);
        }
      } else if (e.keyCode === 40) {
        if (this.$el.nextSibling && this.isFocusedAtLastNode()) {
          e.preventDefault();
          const el = (this.$el.nextSibling! as HTMLElement);
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
      if (this.type === Type.Paragraph) {
        if (this.content.slice(0, 2) === '# ') {
          this.content = this.content.slice(2);
          this.setType(Type.Header);
        } else if (this.content.slice(0, 3) === '## ') {
          this.content = this.content.slice(3);
          this.setType(Type.Header2);
        } else if (/.*(<br\\?>){2,}.*/.test(this.content)) {
          const lines = this.content.split(/<br\\?>/);
          this.content = lines.slice(0, lines.length - 1).join('<br>');
          (this.$refs.input as HTMLElement).innerHTML = this.content;
          this.$emit('new', {content: lines[lines.length - 1], type: this.type});
        } else if (/.*(<br\\?>){2,}$/.test(this.content)) {
          this.content = this.content.substr(0, /(<br\\?>)*$/.exec(this.content)!.index);
          (this.$refs.input as HTMLElement).innerHTML = this.content;
          this.$emit('new');
        }
      } else {
        if (/.*<br\\?>.*/.test(this.content)) {
          const lines = this.content.split(/<br\\?>/);
          this.content = lines[0];
          (this.$refs.input as HTMLElement).innerHTML = this.content;
          this.$emit('new', {content: lines[1], type: this.type});
        } else if (/.*(<br\\?>)$/.test(this.content)) {
          this.content = this.content.substr(0, /(<br\\?>)*$/.exec(this.content)!.index);
          (this.$refs.input as HTMLElement).innerHTML = this.content;
          this.$emit('new');
        }
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
      if (this.type === Type.Paragraph && /<br\\?>/.test(this.content) && t !== Type.Paragraph) {
        const lines = this.content.split(/<br\\?>/);
        this.content = lines[0];
        lines.slice(1).forEach((e: string) => {
          this.$emit('new', {
            content: e,
            type: t,
          });
        });
      }
      this.type = t;
      setTimeout(() => {
        const el = this.$refs.input as HTMLElement;
        if (this.content === '' && this.isFirefox && t !== Type.Paragraph) {
          el.innerHTML = `<br type="_moz">`;
        }
        el.focus();
        setTimeout(() => {
          const val = el.innerHTML;
          el.childNodes.forEach((c) =>
            c.remove(),
          );
          el.innerHTML = val;
          el.blur();
          setTimeout(() => {
            const sel = window.getSelection();
            sel.collapse(el.childNodes[0], offset);
            (this.$refs.input as HTMLElement).focus();
          }, 0);
        }, 0);
      }, 0);
    },
    lastNode() {
      const el = (this.$refs.input as HTMLElement);
      const allNodes = Array.from(el.childNodes).filter((e) => e.nodeName !== 'BR');
      const lastNode = allNodes[allNodes.length - 1];
      return lastNode;
    },
    focusAtEnd() {
      const sel = document.getSelection()!;
      const lastNode = this.lastNode();
      if (lastNode) {
        sel.collapse(lastNode, lastNode.nodeValue!.length);
      }
    },
    isFocusedAtEnd() {
      const sel = document.getSelection()!;
      const lastNode = this.lastNode();
      return lastNode ?
        lastNode.isSameNode(sel.anchorNode)
          && sel.anchorOffset === lastNode.nodeValue!.length
          && sel.isCollapsed
        : true;
    },
    isFocusedAtStart() {
      if ((this.$refs.input as HTMLElement).childNodes.length === 0) {
        return true;
      }
      const sel = document.getSelection()!;
      return ((this.$refs.input as HTMLElement).childNodes[0].isSameNode(sel.anchorNode)
        || (sel.anchorNode.nodeType === 1
        && (sel.anchorNode as HTMLElement).matches('.content')))
        && sel.anchorOffset === 0 && sel.isCollapsed;
    },
    isFocusedAtLastNode() {
      const lastNode = this.lastNode();
      if (!lastNode) {
        return true;
      }
      const sel = document.getSelection()!;
      return (lastNode.isSameNode(sel.anchorNode)) && sel.isCollapsed;
    },
    isFocusedAtFirstNode() {
      const childNodes = (this.$refs.input as HTMLElement).childNodes;
      if (childNodes.length === 0) {
        return true;
      }
      const sel = document.getSelection()!;
      return sel.anchorNode.isSameNode(childNodes[0]) && sel.isCollapsed;
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