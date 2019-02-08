<template lang="pug">
  router-view
</template>

<style lang="sass">
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
h1
  font-weight: 600
p
  line-height: 26px
::selection
  background: var(--accent-color)
  color: var(--secondary-foreground-color)
</style>
