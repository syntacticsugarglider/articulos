<template lang="pug">
  router-view
</template>

<style lang="sass">
*
  box-sizing: border-box
body
  margin: 0
  font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen-Sans,Ubuntu,Cantarell,"Helvetica Neue",sans-serif
  -webkit-font-smoothing: antialiased
  -moz-osx-font-smoothing: grayscale
  color: var(--foreground-color)
  background: var(--background-color)
  font-size: 15px
  text-align: center
  --accent-color: #9c27b0
  --background-color: #fff
  --secondary-accent-color: #ab47bc
  --secondary-foreground-color: #f3e5f5
  --foreground-color: #333
h1, h2
  font-weight: 600
p
  line-height: 26px
::selection
  background: var(--accent-color)
  color: var(--secondary-foreground-color)
</style>

<script lang="ts">
import Vue from 'vue';

declare var InstallTrigger: any;

export default Vue.extend({
  data() {
    return {
    };
  },
  mounted() {
    if (typeof InstallTrigger !== 'undefined') {
      document.addEventListener('keydown', (e) => {
        const target = (e.target || e.srcElement)! as HTMLElement;
        if (e.keyCode === 8 && target.matches('.content')) {
          const sel = document.getSelection()!;
          if (target.parentElement &&
            (target.parentElement as any).__vue__ && (target.parentElement as any).__vue__.isFocusedAtStart()) {
            e.preventDefault();
          }
        }
      });
    }
  },
});
</script>
