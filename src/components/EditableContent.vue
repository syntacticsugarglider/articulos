<template lang="pug">
component.content(:contenteditable="contentType !== 3", :is="element")
  template(v-for="item in content.entries()")
    span(contenteditable, v-if="typeof item[1] === 'string'", v-text="item[1]", :ref="item[0]", :data-key="item[0]")
    br(v-else-if="item[1] === 1", :data-key="item[0]")
    EditableContent(v-else, :ref="item[0]", :data-key="item[0]", v-model="item[1]")
</template>

<script lang="ts">
import Vue from 'vue';

enum Type {
  Editor = 1,
  Paragraph,
  Header,
}

enum Identity {
  LineBreak = 1,
}

interface Item {
  content: Array<Item | Identity | string>;
  type: Type;
}

const ElementForType = new Map<Type, string>();

ElementForType.set(Type.Paragraph, 'p');
ElementForType.set(Type.Editor, 'div');

export default Vue.extend({
  name: 'EditableContent',
  props: ['value'],
  data() {
    const type = this.$props.value ? (this.$props.value as Item).type : Type.Editor;
    return {
      element: ElementForType.get(type),
      content: this.$props.value ? (this.$props.value as Item).content :
        [{content: [String.fromCharCode(65279)], type: Type.Paragraph}] as Array<Item | string>,
      contentType: type,
    };
  },
  mounted() {
    const ZWNBS = String.fromCharCode(65279);
    const mo = new MutationObserver((E) => {
      E.forEach((e) => {
        if (e.type === 'characterData') {
          this.content[parseInt(e.target.parentElement!.getAttribute('data-key')!, 10)]
            = e.target.nodeValue!;
          if (e.target.nodeValue!.includes(ZWNBS)) {
            e.target.nodeValue = e.target.nodeValue!.replace(ZWNBS, '');
            this.moveCaret(1);
          }
        } else if (e.type === 'childList') {
          e.addedNodes.forEach((node) => {
            if (node.nodeName === 'BR' && node.previousSibling && node.previousSibling!.nodeName === 'BR') {
              e.target.parentElement!.appendChild(document.createElement('br'));
              this.content.push(Identity.LineBreak);
              this.content.push(ZWNBS);
              this.$nextTick(() => {
                mo.observe((this.$refs[this.content.length - 1] as Node[])[0], {
                  characterData: true,
                  childList: true,
                  subtree: true,
                });
              });
            }
          });
        }
      });
    });
    for (const item of this.content.entries()) {
      if (typeof item[1] === 'string') {
        mo.observe((this.$refs[item[0]] as Node[])[0], {
          characterData: true,
          childList: true,
          subtree: true,
        });
      }
    }
    new MutationObserver((E) => {
      E.forEach((e) => {
        console.log(e);
      });
    }).observe(this.$el, {
      childList: true,
    });
  },
  methods: {
    focusAtEnd() {
      const sel = document.getSelection()!;
      sel.collapse((this.$refs[this.content.length - 1] as Node).lastChild!, 1);
    },
    moveCaret(offset: number) {
      const sel = document.getSelection()!;
      sel.collapse(sel.anchorNode, sel.anchorOffset + offset);
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