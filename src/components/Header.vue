<template>
  <div class="app-header">
    <div class="logo">
      <div>VRChat缓存转移工具</div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.app-header {
  width: 100%;
  height: 75px;
  .logo {
    user-select: none;
    div {
      width: 330px;
      height: 60px;
      line-height: 60px;
      font-size: 32px;
      cursor: pointer;
      color: var(--text-color);
      text-align: center;
      transition: color 0.5s;
      transform: translate(0%);
    }

    div::before {
      content: "";
      position: absolute;
      left: 0%;
      bottom: 0;
      width: 330px;
      transform: scaleX(0);
      height: 2px;
      background: #ff6266;
      z-index: -1;
      transition: transform 0.5s;
      transform-origin: 100% 0;
    }

    div:hover::before {
      transform: scaleX(1);
      transform-origin: 0 0;
    }
  }
}
</style>
