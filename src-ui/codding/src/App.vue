<template>
  <a-config-provider :locale="locale">
    <div class="codding-layout">
      <div class="top-bar">
        <div>Palm</div>
        <div>设置</div>
      </div>
      <div class="workspace">
        <div class="actions">
          <ul>
            <li><FileAddOutlined /></li>
            <li><BranchesOutlined /></li>
          </ul>
        </div>
        <div class="file-tree">
          <FileTree />
        </div>
        <div class="divider"></div>
        <div class="main">
          <div class="editor-area">
            <CodeEditor />
          </div>
          <div class="interactive">
            <Interactive />
          </div>
        </div>
      </div>
    </div>
  </a-config-provider>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import CodeEditor from './components/CodeEditor.vue';
import { FileAddOutlined, BranchesOutlined } from '@ant-design/icons-vue';
import FileTree from './components/FileTree.vue';
import Interactive from './components/Interactive.vue';
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import dayjs from 'dayjs';
import 'dayjs/locale/zh-cn';
dayjs.locale('zh-cn');
let locale = ref({
  locale: zhCN,
});
</script>
<style lang="scss" scoped>
.codding-layout {
  background: $mainColor;
  min-height: 100vh;
  height: 100vh;
  min-height: 768px;
  min-width: 1366px;
  color: #fff;
  overflow: hidden;

  .top-bar {
    width: 100%;
    height: 41px;
    background-color: $thirdColor;
    border-bottom: 1px solid $deep;
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 24px;
  }

  .divider {
    width: 4px;
    background-color: $secondColor;

    &:hover {
      background-color: $gary;
      cursor: w-resize;
    }
  }

  .workspace {
    display: flex;
    height: calc(100vh - 41px);
  }

  .actions {
    width: 50px;
    height: 100%;
    background-color: $thirdColor;
    border-right: 1px solid $deep;
    overflow-y: overlay;
    text-align: center;

    ul li {
      padding: 10px 0;
      cursor: pointer;

      &:hover {
        background-color: $deep;
      }
    }
  }
  .file-tree {
    width: 250px;
    height: 100%;
    background-color: $deep;
    overflow-y: overlay;
  }

  .main {
    background-color: $mainColor;
    flex: 1;
  }

  .editor-area {
    height: calc(100vh - 361px);
    width: 100%;
    background-color: $thirdColor;
    border-bottom: 1px solid $deep;
    overflow-y: hidden;
  }
}
</style>
