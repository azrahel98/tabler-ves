<template>
  <div class="loading-page">
    <div class="loaderi"></div>
  </div>
</template>

<style lang="scss" scoped>
.loading-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;

  .loaderi {
    width: 50px;
    aspect-ratio: 1;
    border-radius: 50%;
    border: 8px solid var(--tblr-primary);
    border-right-color: var(--tblr-secondary);
    animation: l2 3s infinite linear;
  }
  @keyframes l2 {
    to {
      transform: rotate(1turn);
    }
  }
}
</style>
