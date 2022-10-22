<template>
  <d2-container class="page">
    <d2-page-cover>
      <d2-icon-svg class="logo" name="d2-admin"/>
      <template slot="footer">
        <div class="btn-group">
          <span class="btn-group__btn" @click="$open('https://github.com/d2-projects')">开源组织</span> |
          <span class="btn-group__btn" @click="$open('https://d2.pub/zh/doc/d2-admin')">文档</span> |
          <span class="btn-group__btn" @click="$open('https://github.com/d2-projects/d2-admin-start-kit')">简化版</span> |
          <span class="btn-group__btn" @click="$open('https://juejin.im/user/57a48b632e958a006691b946/posts')">掘金</span> |
          <el-popover :width="150" trigger="hover">
            <img src="./image/qr.jpg" style="width: 150px;">
            <span slot="reference" class="btn-group__btn btn-group__btn--link">
              <d2-icon name="weixin"/>
              微信服务号
            </span>
          </el-popover>
        </div>
        <d2-badge/>
        <d2-help/>
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
      </template>
    </d2-page-cover>
  </d2-container>
</template>

<script>
import D2Badge from './components/d2-badge'
import D2Help from './components/d2-help'
import D2PageCover from './components/d2-page-cover'
export default {
  components: {
    D2Badge,
    D2Help,
    D2PageCover
  },
  data() {
    return {
      urls: [
        { 'name': '个人博客', 'url': 'http://lflxp.gitee.io', 'type': 'success', 'icon': 'el-icon-edit' },
        { 'name': '管理后台', 'url': '/admin/index', 'type': 'primary', 'icon': 'el-icon-share' },
        { 'name': 'd2-admin', 'url': '/d2admin', 'type': 'primary', 'icon': 'el-icon-share' },
        { 'name': 'Swagger', 'url': '/swagger/index.html', 'type': 'warning', 'icon': 'el-icon-search' },
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
  methods: {
    jump(path) {
      window.location.href = path
    }
  }
}
</script>

<style lang="scss" scoped>
.page {
  .logo {
    width: 120px;
  }
  .btn-group {
    color: $color-text-placehoder;
    font-size: 12px;
    line-height: 12px;
    margin-top: 0px;
    margin-bottom: 20px;
    .btn-group__btn {
      color: $color-text-sub;
      &:hover {
        color: $color-text-main;
      }
      &.btn-group__btn--link {
        color: $color-primary;
      }
    }
  }
}
</style>
