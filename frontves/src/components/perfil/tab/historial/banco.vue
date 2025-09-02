<template>
  <div class="card-body p-0 m-0 pb-3">
    <div class="credit-card mt-3">
      <div class="d-flex justify-content-between">
        <h3>{{ item?.banco }}</h3>
        <span>{{ item?.tipo_cuenta }}</span>
      </div>
      <div class="card-chip mt-3"></div>
      <div class="mt-4">
        <span class="card-number fs-5 mb-0">{{ item?.numero_cuenta }}</span>
        <p class="card-number m-0 fs-3">{{ item?.cci }}</p>
      </div>
    </div>
    <div class="form"></div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  item: { type: Object, required: true }
})
</script>

<style lang="scss" scoped>
.card-body {
  display: flex;
  justify-content: center;
  .credit-card {
    display: grid;
    max-width: 15vw;
    grid-template-rows: repeat(3, min-height);
    width: 90%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 16px;
    padding: 1.5rem;
    color: white;
    position: relative;
    overflow: hidden;

    .card-chip {
      width: 40px;
      height: 30px;
      background: linear-gradient(45deg, #ffd700, #ffed4e);
      border-radius: 6px;
      margin-bottom: 1rem;
      position: relative;
      box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .card-chip::before {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 60%;
      height: 60%;
      background: #e6c200;
      border-radius: 2px;
    }
    .card-number {
      font-size: 1.5rem;
      font-weight: 200;
      letter-spacing: 4px;
      margin-bottom: 1.5rem;
    }
  }
  .credit-card::before {
    content: '';
    position: absolute;
    top: -50%;
    right: -50%;
    width: 100%;
    height: 100%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.008) 00%, transparent 100%);
  }
}
</style>
