<template>
  <div class="interactive">
    <div class="tabs">
      <dl>
        <dt>
          <a-tabs v-model:activeKey="state.activeKey">
            <a-tab-pane
              :key="i"
              v-for="(o, i) in state.tabs">
              <template #tab>
                <span class="tab-name"><FileTextOutlined />{{ o.name }}</span>
              </template>
            </a-tab-pane>
          </a-tabs>
        </dt>
        <dd>+</dd>
      </dl>
    </div>
    <div class="bord">
      <Terminal />
    </div>
  </div>
</template>
<script lang="ts" setup>
import { reactive } from 'vue';
import Terminal from './Terminal.vue';

const state = reactive({
  activeKey: '1',
  tabs: [
    { name: '问题', status: false },
    { name: '输出', status: false },
    { name: '调试控制台', status: false },
    { name: '终端', status: false },
    { name: '端口', status: false },
  ],
});
</script>

<style scoped lang="scss">
:deep(.ant-tabs) {
  background-color: $thirdColor;
}

:deep(.ant-tabs-nav) {
  margin-bottom: 0;
  overflow: hidden;
}

:deep(.ant-tabs-nav-operations) {
  background-color: $thirdColor;
}
.interactive {
  height: 300px;
  background-color: $thirdColor;
  border-right: 1px solid $deep;
  overflow-y: overlay;

  .tabs dl {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 24px;
  }

  .tab-name {
    color: $white;
  }

  .bord {
    background-color: $mainColor;
    height: calc(100% - 46px);
  }
}
</style>
