<template>
  <div>
    <t-layout>
      <t-aside class="fixed-aside">
        <div class="file-list-container">
          <div class="file-list">
            <div 
              v-for="(item, index) in fileList" 
              :key="index"
              class="file-item" 
            >
              <span class="file-path" 
                :class="{ 'active': activeFilePath === item.path }"  
                @click="getEnvList(item.path); 
                activeFilePath = item.path">
              {{ item.path }}
              </span>
              <t-icon v-if="index < fileList.length - 1" name="chevron-down" class="arrow-down" />
            </div>
          </div>
          <div class="aside-footer">
            <t-button theme="default" size="small" class="footer-btn" @click="showShellSelector = true">
              <t-icon name="swap" />
              <span>切换Shell</span>
            </t-button>
            <t-button theme="default" size="small" class="footer-btn" @click="showSettings = true">
              <t-icon name="setting" />
              <span>设置</span>
            </t-button>
          </div>
        </div>
      </t-aside>
      <t-layout class="content-layout">
        <t-content class="scrollable-content" v-if="envList.length > 0">
          <t-list class="env-list">
            <t-list-item v-for="(item, index) in envList" :key="index" class="env-list-item">
              <div class="env-card-wrapper">
                <t-card :bordered="false" class="full-width-card">
                  <div class="card-header">
                      <div class="env-key">{{ item.key }}</div>                  
                      <t-button class="add-btn" theme="primary" variant="text" size="small" @click="addValue(item)">
                        <t-icon name="add" />
                      </t-button>                      
                  </div>


                  <t-space direction="vertical" style="width: 100%">
                    <div v-for="(_, vIndex) in item.value" :key="vIndex" class="input-wrapper">
                      <t-input
                        v-model="item.value[vIndex]"
                        class="value-input" 
                        @change="updateEnvVariable(item)"
                      />
                      <t-button 
                        theme="danger" 
                        variant="text" 
                        shape="circle" 
                        class="delete-btn"
                        @click="removeValue(item, vIndex)"
                      >
                        <t-icon name="close" />
                      </t-button>
                    </div>
                  </t-space>
                </t-card>
              </div>
            </t-list-item>
          </t-list>
          <div class="floating-button">
            <t-button theme="primary" shape="circle" size="large" @click="showAddDialog = true">
              <t-icon name="add" />
            </t-button>
            <t-button theme="primary" shape="circle" size="large" @click="saveEnvVariable">
              <t-icon name="save" />
            </t-button>
          </div>

          <AddEnvDialog
            v-model:visible="showAddDialog"
            @confirm="handleAddEnv"
          />
        </t-content>

        <t-content class="scrollable-content-none" v-else>
          <t-empty>
            <template #icon>
              <t-icon name="smile" />
            </template>
          </t-empty>
        </t-content>


      </t-layout>
    </t-layout>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
//引入类型文件 
import { ApiResponse, ConfigFile,EnvVariableResponse } from '../types/rust-type';
import AddEnvDialog from './AddEnvDialog.vue'

// 定义一个变量，文件列表
const fileList = ref<ConfigFile[]>([])
const activeFilePath = ref<string>('')
const showShellSelector = ref<boolean>(false)
const showSettings = ref<boolean>(false)

const getFileList = async () => {
  const result = await invoke<ApiResponse<any>>('get_config_files');
  if (result) {
    fileList.value = result.data;
    if (fileList.value.length > 0 && !activeFilePath.value) {
      activeFilePath.value = fileList.value[0].path;
      getEnvList(activeFilePath.value);
    }
  }
}

//获取一个文件里的所有变量
const envList = ref<EnvVariableResponse[]>([])
const getEnvList = async (filePath: string) => {
  console.log('获取文件列表:', filePath);
  
  const result = await invoke<ApiResponse<any>>('load_config_file_content', { filePath });
  if (result.code === 200) {
    console.log('获取文件列表成功:', result.data);
    
    envList.value = result.data;
  } else {
    //在界面上弹出错误
    console.error('获取文件列表失败:', result.message);
    envList.value = [];
  }
}
onMounted(() => {
  getFileList()
})

// 更新保存环境变量的方法
const updateEnvVariable = async (item: any) => {
  try {
    const result = await invoke<ApiResponse<any>>('update_env_variable', {
      envVar: {
        index: item.index,
        key: item.key,
        value: item.value
      }
    });
    if (result.code === 200) {
      // 可以添加一个成功提示
      console.log('更新成功');
    }
  } catch (error) {
    console.error('更新失败:', error);
  }
}
// 保存环境变量的方法
const saveEnvVariable = async () => {
  try {
    const result = await invoke<ApiResponse<any>>('save_config_file');
    if (result.code === 200) {
      // 可以添加一个成功提示
      console.log('保存成功');
    }
  } catch (error) {
    console.error('保存失败:', error);
  }
}
const showAddDialog = ref(false)

const handleAddEnv = async (env: any) => {
  console.log('添加环境变量:', env);
  
  try {
    const result = await invoke<ApiResponse<any>>('add_env_variable', {
      envVar: {
        index: 0,
        key: env.key,
        value: env.value
      }
    })
    if (result.code === 200) {
      //保存
      saveEnvVariable()
      // 刷新列表
      await getEnvList(activeFilePath.value)
    }
  } catch (error) {
    console.error('添加失败:', error)
  }
}
const addValue = (item: any) => {
  item.value.push('')
  updateEnvVariable(item)
}

const removeValue = (item: any, index: number) => {
  item.value.splice(index, 1)
  updateEnvVariable(item)
}
</script>
<style>
.t-layout {
  height: 100vh;
  width: 100%;
}


.fixed-aside {
  min-width: 200px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--td-component-stroke, rgba(0, 0, 0, 0.1));
  background-color: var(--td-bg-color-container);
}

.file-list-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px 0;
}

.file-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  overflow-y: auto;
  padding: 0 16px;
}

.file-item {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 8px 0;
}

.file-path {
  cursor: pointer;
  padding: 6px 12px;
  border-radius: 4px;
  text-align: center;
  width: 100%;
  transition: all 0.2s;
  color: var(--td-text-color-primary);
}

.file-path:hover {
  transform: scale(0.95); /* 使用缩放实现视觉上的宽度变化 */
  background-color: var(--td-bg-color-container-hover);
}

.file-path.active {
  transform: scale(0.9); /* 比hover状态稍小 */
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1); /* 添加层次感 */
  background-color: var(--td-brand-color-light);
  color: var(--td-brand-color);
  font-weight: 500;
}

.arrow-down {
  margin-top: 8px;
  color: var(--td-text-color-placeholder);
}

.aside-footer {
  margin-top: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  border-top: 1px solid var(--td-component-stroke);
}

.footer-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.aside-footer {
  margin-top: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.footer-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.content-layout {
  overflow: hidden;
}

.scrollable-content {
  height: 100%;
  overflow-y: auto;
  /* padding: 16px; */
}

.scrollable-content-none {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-container {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.env-card {
  margin: 10px 0;
  width: 100%;
}

.key-input {
  flex: 1;
}

.value-input {
  width: 100%;
}

.env-list {
  padding: 16px;
}

.env-list-item {
  margin-bottom: 16px;
}

.env-card-wrapper {
  width: 100%;
}

.full-width-card {
  width: 100%;
  margin-bottom: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);  /* 保持与普通状态相同的阴影 */
}



.floating-button {
  position: fixed;
  bottom: 22px;
  right: 22px;
  z-index: 100;
  display: flex;
  gap: 12px;  /* 添加按钮之间的间距 */
}

.floating-button .t-button {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
.input-wrapper {
  display: flex;
  align-items: center;
  width: 100%;
  margin-bottom: 8px;
}
.value-input {
  width: 100%;
  margin-bottom: 8px;
}
.value-input {
  flex: 1;
  margin-right: 8px;
}

.delete-btn {
  flex-shrink: 0;
}

.card-header {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 5px 0px 10px 0px;  /* 上、左右、下的内边距 */
  background-color: transparent;
}

.env-key {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: var(--td-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.add-btn {
  flex-shrink: 0;
  margin-left: 8px;
}
</style>

