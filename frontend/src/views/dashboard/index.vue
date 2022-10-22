<template>
  <div class="dashboard-container">
    <div class="dashboard-text">name: {{ username }}</div>
    <div class="dashboard-text">userid: {{ userid }}</div>
    <div class="dashboard-text">email: {{ email }}</div>
    <br/>
    <el-row :gutter="20">
      <el-col style="margin: 10px 10px;" :xs="12" :sm="6" :md="4" :lg="4" :xl="2" v-for="data in urls" :key="data.name">
        <el-button
          v-if="data.func === undefined"
          :icon="data.icon"
          @click="jump(data.url)"
          :type="data.type">{{ data.name }}</el-button>
        <el-button
          v-else
          :icon="data.icon"
          @click="funcs(data.func)"
          :type="data.type">{{ data.name }}</el-button>
      </el-col>
    </el-row>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import { userinfo } from '@/api/user'
import { invoke } from '@tauri-apps/api'

export default {
  name: 'Dashboard',
  computed: {
    ...mapGetters([
      'name'
    ])
  },
  data() {
    return {
      userid: '',
      username: '',
      email: '',
      ip: '',
      button: '',
      urls: [
        { 'name': '个人博客', 'url': 'http://lflxp.gitee.io', 'type': 'success', 'icon': 'el-icon-edit' },
        { 'name': '管理后台', 'url': '/admin/index', 'type': 'primary', 'icon': 'el-icon-share' },
        { 'name': 'd2-admin', 'url': '/d2admin', 'type': 'primary', 'icon': 'el-icon-share' },
        { 'name': 'Swagger', 'url': '/swagger/index.html', 'type': 'warning', 'icon': 'el-icon-search' },
        { 'name': 'tauri测试', 'func': 'getips' ,'type': 'danger', 'icon': 'el-icon-view' },
        { 'name': '获取IP', 'func': 'buttons', 'type': 'primary', 'icon': 'el-icon-search' },
        { 'name': 'Notion', 'url': 'https://www.notion.so/', 'type': 'warning', 'icon': 'el-icon-eleme' },
        { 'name': 'tools.fun', 'url': 'https://tools.fun/', 'type': 'info', 'icon': 'el-icon-eleme' },
        { 'name': '哔哩哔哩', 'url': 'https://bilibili.com/', 'type': 'primary', 'icon': 'el-icon-eleme' },
        { 'name': 'Kubernetes', 'url': 'https://kubernetes.io/zh-cn/docs/', 'type': 'danger', 'icon': 'el-icon-eleme' },
        { 'name': 'Rust操作系统', 'url': 'https://www.bilibili.com/medialist/play/1286472?from=space&business=space_series&business_id=965750&desc=1', 'type': 'success', 'icon': 'el-icon-eleme' },
        { 'name': '刘耕宏健身', 'url': 'https://search.bilibili.com/all?keyword=%E5%88%98%E7%95%8A%E5%AE%8F&from_source=webtop_search&spm_id_from=333.1007&search_source=5', 'type': 'warning', 'icon': 'el-icon-eleme' },
        { 'name': 'Element', 'url': 'https://element.eleme.cn/#/zh-CN/component/installation', 'type': 'success', 'icon': 'el-icon-star-on' },
        { 'name': 'dumi', 'url': 'https://d.umijs.org/zh-CN', 'type': 'success', 'icon': 'el-icon-star-on' },
        { 'name': 'umijs', 'url': 'https://umijs.org/', 'type': 'success', 'icon': 'el-icon-star-on' },
        { 'name': 'bootstrap3', 'url': 'https://getbootstrap.com/docs/3.4/', 'type': 'danger', 'icon': 'el-icon-star-on' },
        { 'name': '科技爱好者周刊', 'url': 'https://github.com/ruanyf/weekly', 'type': 'danger', 'icon': 'el-icon-star-on' },
      ]
    }
  },
  mounted() {
    userinfo().then(resp => {
      console.log(resp)
      this.userid = resp.userid
      this.username = resp.username
      this.email = resp.email
    })
  },
  methods: {
    funcs(t) {
      if (t === 'getips') {
        this.getips()
      } else if (t === 'buttons') {
        this.buttons()
      }
    },
    getips() {
      invoke('getip')
      .then((resp) => {
        console.log('getip', resp);
        this.ip = resp
      })
      .catch((err) => {
        console.error('err', err);
        alert('err ' + err);
      });
    },
    buttons() {
      invoke('greet', { name: 'Welcome' }).then((resp) => {
        console.log('resp', resp);
        this.button = resp
      })
    },
    jump(path) {
      window.location.href = path
    }
  }
}
</script>

<style lang="scss" scoped>
.dashboard {
  &-container {
    margin: 30px;
  }
  &-text {
    font-size: 30px;
    line-height: 46px;
  }
}
</style>
