<template>
  <div class="code-editor">
    <div class="file-tabs">
      <a-tabs
        v-model:activeKey="activeKey"
        @tabScroll="callback"
        @edit="onEdit">
        <a-tab-pane
          :key="i"
          v-for="(o, i) in state.tabs">
          <template #tab>
            <span class="tab-name"><FileTextOutlined />{{ o.name }}{{ i }}</span>
          </template>
        </a-tab-pane>
        <template #moreIcon>
          <EllipsisOutlined class="more-icon" />
        </template>
      </a-tabs>
    </div>
    <div class="editor">
      <div ref="codeEditor"></div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue';
import { FileTextOutlined, EllipsisOutlined } from '@ant-design/icons-vue';
import PalmEditor, { LanguageType, Themes } from 'palm-editor/dist/index.es';
import { monokai } from '@uiw/codemirror-theme-monokai';
import { monokaiDimmed } from '@uiw/codemirror-theme-monokai-dimmed';
import { EditorView, keymap, placeholder } from '@codemirror/view';
const codeEditor = ref();
const activeKey = ref(1);
const editor = ref();
const state = reactive({
  codeText: '',
  tabs: [
    { name: '文件', status: false },
    { name: '文件', status: false },
    { name: '文件', status: false },
    { name: '文件', status: false },
    { name: '文件', status: false },
    { name: '文件', status: false },
  ],
});

// const myTheme = createTheme({
//   theme: 'light',
//   settings: {
//     background: '#ffffff',
//     backgroundImage: '',
//     foreground: '#75baff',
//     caret: '#5d00ff',
//     selection: '#036dd626',
//     selectionMatch: '#036dd626',
//     lineHighlight: '#8a91991a',
//     gutterBorder: '1px solid #ffffff10',
//     gutterBackground: '#fff',
//     gutterForeground: '#8a919966',
//   },
//   styles: [
//     { tag: t.comment, color: '#787b8099' },
//     { tag: t.variableName, color: '#0080ff' },
//     { tag: [t.string, t.special(t.brace)], color: '#5c6166' },
//     { tag: t.number, color: '#5c6166' },
//     { tag: t.bool, color: '#5c6166' },
//     { tag: t.null, color: '#5c6166' },
//     { tag: t.keyword, color: '#5c6166' },
//     { tag: t.operator, color: '#5c6166' },
//     { tag: t.className, color: '#5c6166' },
//     { tag: t.definition(t.typeName), color: '#5c6166' },
//     { tag: t.typeName, color: '#5c6166' },
//     { tag: t.angleBracket, color: '#5c6166' },
//     { tag: t.tagName, color: '#5c6166' },
//     { tag: t.attributeName, color: '#5c6166' },
//   ],
// });
const extensions = [monokai];

onMounted(() => {
  editor.value = new PalmEditor(codeEditor.value, state.codeText, {
    view: {
      placeholder: '请输入',
      change: (val: string) => {
        console.log(val, '值改变执行');
      },
      theme: Themes.monokai,
      // extensions: extensions,
    },
    state: {
      // extensions: [EditorView.theme(monokai, { dark: false })],
    },
    type: LanguageType.javascript,
  });
  console.log(editor, 'editor');
});

const callback = () => {
  console.log('callback');
};

const onEdit = (targetKey: any, action: any) => {
  if (action === 'add') {
    // add();
  } else {
    console.log(targetKey);
    // remove(targetKey as string);
  }
};

const getData = (type: number) => {
  if (type == 1) {
    return editor.value.getSelection();
  }
};

// 初始化
// 主题样式设置
// {
//   '&': {
//     color: 'white',
//     backgroundColor: '#1e1e1e',
//   },
//   '.cm-content': {
//     caretColor: '#1e1e1e',
//   },
//   '&.cm-focused .cm-cursor': {
//     borderLeftColor: '#1e1e1e',
//   },
//   '&.cm-focused .cm-selectionBackground, ::selection': {
//     backgroundColor: '#1e1e1e',
//     color: '#FFF',
//   },
//   '.cm-gutters': {
//     backgroundColor: '#1e1e1e',
//     color: '#74715e',
//     border: 'none',
//   },
//   '.cm-line': {
//     // borderBottom: '1px solid #f00',
//   },
//   'cm-s-cobalt': {
//     borderLeft: '1px solid #30aaed !important',
//   },
// };

defineExpose({
  getData,
});
</script>
<style lang="scss" scoped>
.cm-s-cobalt .CodeMirror-cursor {
  border-left: 1px solid #30aaed !important;
}
:deep(.ant-tabs) {
  background-color: $mainColor;
}

:deep(.ant-tabs-nav) {
  margin-bottom: 0;
  overflow: hidden;
}
.code-editor {
  position: relative;
  height: calc(100%);
  overflow: hidden;
}

.editor {
  height: (calc(100% - 40px));
  overflow: hidden;
  overflow-y: auto;
  box-sizing: border-box;
  padding: 5px;
}
.file-tabs {
  width: calc(100vw - 300px);
  box-sizing: border-box;
  padding-left: 12px;
}

.tab-name {
  color: $white;
  padding: 0 5px;
}

:deep(.ant-tabs-nav-operations) {
  background-color: $thirdColor;
}

.more-icon {
  font-size: 30px;
  color: #fff;
}

.code-editor {
  background: $mainColor;
}
</style>
