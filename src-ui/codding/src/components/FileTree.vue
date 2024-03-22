<template>
  <a-menu
    id="file-tree"
    v-model:openKeys="openKeys"
    v-model:selectedKeys="selectedKeys"
    mode="inline"
    :items="items"
    :style="headerStyle"
    @click="handleClick"></a-menu>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
const headerStyle: CSSProperties = {
  textAlign: 'center',
  color: '#fff',
  height: 64,
  paddingInline: 50,
  lineHeight: '64px',
  backgroundColor: '#1e1e1e',
};
import { reactive, ref, watch, VueElement, h } from 'vue';
import { FileTextOutlined } from '@ant-design/icons-vue';
import type { MenuProps, ItemType } from 'ant-design-vue';

const selectedKeys = ref<string[]>(['1']);
const openKeys = ref<string[]>(['sub1']);

function getItem(label: VueElement | string, key: string, icon?: any, children?: ItemType[], type?: 'group'): ItemType {
  return {
    key,
    icon,
    children,
    label,
    type,
  } as ItemType;
}

const items: ItemType[] = reactive([
  getItem('componets', 'sub1', () => h(FileTextOutlined), [
    getItem('Item 1', 'g1', () => h(FileTextOutlined)),
    getItem('Item 2', 'g1', () => h(FileTextOutlined)),
    getItem('Item 3', 'g1', () => h(FileTextOutlined)),
    getItem('Item 4', 'g1', () => h(FileTextOutlined)),
    getItem('Item 5', 'g1', () => h(FileTextOutlined)),
    getItem('Item 6', 'g1', () => h(FileTextOutlined)),
  ]),
]);

const handleClick: MenuProps['onClick'] = (e) => {
  console.log('click', e);
};

watch(openKeys, (val) => {
  console.log('openKeys', val);
});
</script>
<style lang="scss" scoped>
.file-tree {
  background-color: #034 !important;
}
</style>
