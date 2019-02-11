<template lang="pug">
component.content(contenteditable, :is="element")
  template(v-for="item in content.entries()")
    EditableContent(v-if="typeof item[1] !== 'string'", :ref="item[0]", :data-key="item[0]", :key="item[0]")
    span(contenteditable, v-else, v-text="item[1]", :ref="item[0]", :data-key="item[0]", :key="item[0]")
</template>

<script lang="ts">
import Vue from 'vue';

enum Type {
  Paragraph = 1,
  Header,
}

interface Item {
  content: Array<Item | string>;
  type: Type;
}

const ElementForType = new Map<Type, string>();

ElementForType.set(Type.Paragraph, 'p');

export default Vue.extend({
  name: 'EditableContent',
  data() {
    return {
      element: 'p',
      content: [] as Array<Item | string>,
      type: Type.Paragraph,
    };
  },
  mounted() {
    const ZWNBS = String.fromCharCode(65279);
    this.appendContent(ZWNBS);
    new MutationObserver((E) => {
      E.forEach((e) => {
        if (e.type === 'characterData') {
          if ((e.target.parentElement as HTMLElement).isSameNode(this.$el)) {
            this.appendContent(e.target.nodeValue!);
            (e.target as Element).remove();
            this.$nextTick(() => {
              this.moveCarat(e.target.nodeValue!.length);
            });
            return;
          }
          this.content[parseInt((e.target.parentElement as HTMLElement).getAttribute('data-key')!, 10)] =
            e.target.nodeValue!;
          if (e.target.nodeValue!.includes(ZWNBS)) {
            e.target.nodeValue = e.target.nodeValue!.replace(ZWNBS, '');
            this.moveCarat(1);
          }
        } else if (e.type === 'childList') {
          e.removedNodes.forEach((el) => {
            if (el.nodeType === 3) {
              return;
            }
            this.content.splice(parseInt((el as any).attributes.getNamedItem('data-key'), 10), 1);
          });
          e.addedNodes.forEach((el) => {
            if (el.nodeName === 'BR' && el.parentElement!.firstChild!.isSameNode(el)) {
              (el as Element).remove();
            }
          });
        }
      });
    }).observe(this.$el, {
      childList: true,
      characterData: true,
      subtree: true,
    });
    this.focus();
  },
  methods: {
    focus() {
      (this.$el as HTMLElement).focus();
    },
    appendContent(content: Item | string) {
      this.content.push(content);
    },
    moveCarat(offset: number) {
      const sel = document.getSelection()!;
      sel.collapse(sel.anchorNode, sel.anchorOffset + offset);
    },
    insertContent(index: number, content: Item | string) {
      this.content.splice(index, 0, content);
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
p.content::before
  content: "¶"
  margin-top: -1.5px
h1.content::before
  content: "#"
h2.content::before
  content: "##"
  left: -2em
</style>