<template>
  <t-dialog v-model:visible="dialogVisible" header="添加环境变量" :on-confirm="handleConfirm" :on-close="handleClose" class="centered-dialog">
    <t-card :bordered="false" class="full-width-card">
      <div class="card-header">
        <div class="env-key-input">
          <t-form-item label="变量名">
            <t-input v-model="envData.key" placeholder="请输入环境变量名" class="key-input" />
          </t-form-item>
        </div>
      </div>
      <t-space direction="vertical" style="width: 100%">
        <div v-for="(_, index) in envData.value" :key="index" class="input-wrapper">
          <t-form-item :label="index === 0 ? '变量值' : ''" class="value-form-item">
            <t-input v-model="envData.value[index]" placeholder="请输入环境变量值" class="value-input" />
          </t-form-item>
          <t-button 
            v-if="index > 0"
            theme="danger" 
            variant="text" 
            shape="circle" 
            class="delete-btn"
            @click="removeValue(index)"
          >
            <t-icon name="close" />
          </t-button>
        </div>
        <div class="add-value-btn-wrapper">
          <t-button theme="primary" variant="text" size="small" @click="addValue">
            <t-icon name="add" /> 添加值
          </t-button>
        </div>
      </t-space>
    </t-card>
  </t-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps(['visible'])
const emit = defineEmits(['update:visible', 'confirm'])

// 使用计算属性来处理 v-model
const dialogVisible = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value)
})

const envData = ref({
  key: '',
  value: ['']
})

const addValue = () => {
  envData.value.value.push('')
}

const removeValue = (index: number) => {
  envData.value.value.splice(index, 1)
}

const handleConfirm = () => {
  // 过滤掉空值
  const filteredValues = envData.value.value.filter(val => val.trim() !== '')
  if (filteredValues.length === 0) {
    // 确保至少有一个值，即使是空的
    filteredValues.push('')
  }
  envData.value.value = filteredValues
  
  emit('confirm', envData.value)
  handleClose()
  // 重置表单
  resetForm()
}

const handleClose = () => {
  emit('update:visible', false)
  // 重置表单
  resetForm()
}

const resetForm = () => {
  envData.value = {
    key: '',
    value: ['']
  }
}
</script>

<style>
.centered-dialog {
  display: flex;
  align-items: center;
  justify-content: center;
}

.centered-dialog :deep(.t-dialog__panel) {
  max-width: 500px;
  width: 90%;
}

.full-width-card {
  width: 100%;
  margin-bottom: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-header {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 5px 0px 10px 0px;
  background-color: transparent;
}

.env-key-input {
  flex: 1;
  font-size: 16px;
  color: var(--td-text-color-primary);
}

.key-input {
  width: 100%;
}

.input-wrapper {
  display: flex;
  align-items: center;
  width: 100%;
  margin-bottom: 8px;
}

.value-input {
  width: 100%;
}

.value-form-item {
  width: 100%;
  flex: 1;
  margin-bottom: 0;
}

.delete-btn {
  margin-left: 8px;
  font-size: 16px;
  display: flex;
  align-items: center;
}

.add-value-btn-wrapper {
  display: flex;
  justify-content: flex-start;
  margin-top: 8px;
}
</style>