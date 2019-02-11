<template lang="pug">
  .container(@keydown="keydown")
    component.content(ref="input", contenteditable=true, @input="update", :is="typeString")
      template(v-for="sector in sectors.entries()")
        template &#x200B;
        template(v-if="sector[1].type === 1")
          template(v-html="sector[1].content")
        template(v-else-if="sector[1].type === 2")
          span.bold(contenteditable=true, :key="sector[0]", v-html="sector[1].content")
        template &#x200B;
</template>

<script lang="ts">
interface Section {
  type: number;
  content: string;
}

enum Type {
  Paragraph = 1,
  Header,
  SubHeader,
}

const typeMap = new Map<Type, string>();

typeMap.set(Type.Paragraph, 'p');
typeMap.set(Type.Header, 'h1');
typeMap.set(Type.SubHeader, 'h2');

import Vue from 'vue';

declare var InstallTrigger: any;

export default Vue.extend({
  props: ['value'],
  data() {
    return {
      sectors: [{type: 1, content: this.$props.value.content}],
      type: this.$props.value.type,
      typeString: 'p',
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
          this.sectors.push({type: 2, content: '&#x200B;'});
          this.setType(Type.Paragraph);
          setTimeout(() => {
            this.focusAtSectorStart(this.sectors.length - 1);
          }, 5);
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
    update(e: Event) {
      this.sectors = [{type: 1, content: (this.$refs.input as HTMLElement).innerHTML}];
      this.sectors[0].content = this.sectors[0].content
        .replace('&nbsp;', ' ')
        .replace(/^\s*/, '')
        .replace(/<br\\?>(?!<br\\?>)$/, '');
      if (this.sectors[0].content === '' && this.isFirefox) {
        this.sectors[0].content = `<br type="_moz">`;
      }
      if (this.type === Type.Paragraph) {
        if (this.sectors[0].content.slice(0, 2) === '# ') {
          this.sectors[0].content = this.sectors[0].content.slice(2);
          this.setType(Type.Header);
        } else if (this.sectors[0].content.slice(0, 3) === '## ') {
          this.sectors[0].content = this.sectors[0].content.slice(3);
          this.setType(Type.SubHeader);
        } else if (/.*(<br\\?>){2,}.*/.test(this.sectors[0].content)) {
          const lines = this.sectors[0].content.split(/<br\\?>/);
          this.sectors[0].content = lines.slice(0, lines.length - 1).join('<br>');
          (this.$refs.input as HTMLElement).innerHTML = this.sectors[0].content;
          this.$emit('new', {content: lines[lines.length - 1], type: this.type});
        } else if (/.*(<br\\?>){2,}$/.test(this.sectors[0].content)) {
          this.sectors[0].content =
            this.sectors[0].content.substr(0, /(<br\\?>)*$/.exec(this.sectors[0].content)!.index);
          (this.$refs.input as HTMLElement).innerHTML = this.sectors[0].content;
          this.$emit('new');
        }
      } else {
        if (/.*<br\\?>.*/.test(this.sectors[0].content)) {
          const lines = this.sectors[0].content.split(/<br\\?>/);
          this.sectors[0].content = lines[0];
          (this.$refs.input as HTMLElement).innerHTML = this.sectors[0].content;
          this.$emit('new', {content: lines[1], type: this.type});
        } else if (/.*(<br\\?>)$/.test(this.sectors[0].content)) {
          this.sectors[0].content =
            this.sectors[0].content.substr(0, /(<br\\?>)*$/.exec(this.sectors[0].content)!.index);
          (this.$refs.input as HTMLElement).innerHTML = this.sectors[0].content;
          this.$emit('new');
        }
      }
      (this.$refs.input as Node).normalize();
      this.$emit('input', {content: this.sectors[0].content, type: this.type});
    },
    setType(t: Type) {
      let offset = 0;
      if (t === Type.Paragraph) {
        if (this.type === Type.Header) {
          this.sectors[0].content = '#' + this.sectors[0].content;
          offset = 1;
        } else if (this.type === Type.SubHeader) {
          this.sectors[0].content = '##' + this.sectors[0].content;
          offset = 2;
        }
      }
      if (this.type === Type.Paragraph && /<br\\?>/.test(this.sectors[0].content) && t !== Type.Paragraph) {
        const lines = this.sectors[0].content.split(/<br\\?>/);
        this.sectors[0].content = lines[0];
        lines.slice(1).forEach((e: string) => {
          this.$emit('new', {
            content: e,
            type: t,
          });
        });
      }
      this.type = t;
      this.typeString = typeMap.get(this.type)!;
      setTimeout(() => {
        const el = this.$refs.input as HTMLElement;
        if (this.sectors[0].content === '' && this.isFirefox && t !== Type.Paragraph) {
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
    focusAtSectorStart(sector: number) {
      const childNodes = (this.$refs.input as HTMLElement).childNodes;
      const sel = document.getSelection()!;
      if (this.sectors[sector]) {
        if (this.sectors[sector].type === 1) {
          sel.collapse(childNodes[sector], 0);
        } else if (this.sectors[sector].type === 2) {
          (childNodes[sector].childNodes[0].parentElement as HTMLElement).focus();
          sel.collapse(childNodes[sector].childNodes[0], 1);
        }
      }
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
  opacity: 0.75
.content:focus::before
  opacity: 1
h1.content::before
  content: "#"
h2.content::before
  content: "##"
  left: -2em
p.content::before
  content: "¶"
  margin-top: -1.5px
.bold
  font-weight: 600
.bold::before
  content: "*"
  cursor: text
  color: var(--secondary-accent-color)
  opacity: 0.75
  transition: opacity 0.15s ease
.bold::after
  content: "*"
  cursor: text
  color: var(--secondary-accent-color)
  opacity: 0.75
  transition: opacity 0.15s ease
.content:focus .bold::after
  opacity: 1
.content:focus .bold::before
  opacity: 1
</style>