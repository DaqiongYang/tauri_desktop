<template>
  <div class="box">
    <div id="xterm"></div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted, onBeforeUnmount } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';
import 'xterm/lib/xterm.js';
const state = reactive({
  term: null as any,
  socket: {} as any,
  WebSocketUrl: 'ws://127.0.0.1:8080', //ws接口
  // 心跳
  lockReconnect: false, //是否真正建立连接
  timeout: 28 * 1000, //30秒一次心跳
  timeoutObj: null as any, //心跳心跳倒计时
  serverTimeoutObj: null as any, //心跳倒计时
  timeoutnum: null as any, //断开 重连倒计时
});

onMounted(() => {
  init(state.WebSocketUrl);
});

// 心跳函数--------------
function reconnect() {
  if (state.lockReconnect) {
    return;
  }
  state.lockReconnect = true;
  //没连接上会一直重连，设置延迟避免请求过多
  state.timeoutnum && clearTimeout(state.timeoutnum);
  state.timeoutnum = setTimeout(function () {
    //新连接
    init(state.WebSocketUrl);
    state.lockReconnect = false;
  }, 2000);
}

function reset() {
  //清除时间
  clearTimeout(state.timeoutObj);
  clearTimeout(state.serverTimeoutObj);
  //重启心跳
  start();
}

function start() {
  //开启心跳
  state.timeoutObj && clearTimeout(state.timeoutObj);
  state.serverTimeoutObj && clearTimeout(state.serverTimeoutObj);
  state.timeoutObj = setTimeout(function () {
    //这里发送一个心跳，后端收到后，返回一个心跳消息，
    if (state.socket.readyState == 1) {
      //如果连接正常
      state.socket.send('ping');
    } else {
      //否则重连
      reconnect();
    }
    state.serverTimeoutObj = setTimeout(function () {
      //超时关闭
      close();
    }, state.timeout);
  }, state.timeout);
}

function initXterm() {
  if (state.term) {
    state.term.dispose();
  }
  state.term = new Terminal({
    // rendererType: 'canvas', //渲染类型
    rows: 35, //行数
    convertEol: true, //启用时，光标将设置为下一行的开头
    scrollback: 10, //终端中的回滚量
    disableStdin: false, //是否应禁用输入
    cursorStyle: 'underline', //光标样式
    cursorBlink: true, //光标闪烁
    theme: {
      foreground: 'yellow', //字体
      background: '#060101', //背景色
      cursor: 'help', //设置光标
    },
  });
  state.term.open(document.getElementById('xterm'));
  const fitAddon = new FitAddon();
  state.term.loadAddon(fitAddon);
  // 支持输入与粘贴方法
  state.term.onData(function (key) {
    //这里key值是你输入的值，数据格式order一定要找后端要！！！！
    let order = {
      data: key,
      operation: 'stdin',
    };
    state.socket.onsend(JSON.stringify(order)); //转换为字符串
  });
}

function init(url) {
  // 实例化socket
  state.socket = new WebSocket(url);
  // 监听socket连接
  state.socket.onopen = open;
  // 监听socket错误信息
  state.socket.onerror = error;
  // 监听socket消息
  state.socket.onmessage = getMessage;
  // 发送socket消息
  state.socket.onsend = send;
}

function open() {
  console.log('socket连接成功');
  initXterm();
  //开启心跳
  start();
}

function error() {
  console.log('连接错误');
  //重连
  reconnect();
}

function close() {
  state.socket.close();
  console.log('socket已经关闭');
  //重连
  reconnect();
}

function getMessage(msg) {
  //msg是返回的数据
  msg = JSON.parse(msg.data);
  state.socket.send('ping'); //有事没事ping一下，看看ws还活着没
  //switch用于处理返回的数据，根据返回数据的格式去判断
  switch (msg['operation']) {
    case 'stdout':
      state.term.write(msg['data']); //这里write也许不是固定的，失败后找后端看一下该怎么往term里面write
      break;
    default:
      console.error('Unexpected message type:', msg); //但是错误是固定的。。。。
  }
  //收到服务器信息，心跳重置
  reset();
}

function send(order) {
  state.socket.send(order);
}

onBeforeUnmount(() => {
  close();
  clearTimeout(state.timeoutObj);
  clearTimeout(state.serverTimeoutObj);
  clearTimeout(state.timeoutnum);
});
</script>

<style lang="scss" scoped>
.box {
  width: 100%;
  height: 100%;
  #xterm {
    width: 100%;
    height: 100%;
  }
}
</style>
