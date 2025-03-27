<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

interface EnvVariable {
  key: string;
  values: string[];
}

interface EnvFile {
  id: string;
  name: string;
  path: string;
  priority: number;
  is_active: boolean;
  variables: EnvVariable[];
}

const envFiles = ref<EnvFile[]>([]);
const isLoading = ref(true);
const errorMessage = ref('');

const selectedFile = ref<EnvFile | null>(null);
const searchQuery = ref('');
const editingVariable = ref<{key: string, index: number} | null>(null);
const newValue = ref('');
const newVariableKey = ref('');
const newVariableValue = ref('');

// 加载配置文件列表
onMounted(async () => {
  try {
    isLoading.value = true;
    const response = await invoke<{code: number, data: EnvFile[]}>('get_config_files');
    if (response.code === 200 && response.data) {
      envFiles.value = response.data;
    } else {
      errorMessage.value = '加载配置文件失败';
    }
  } catch (error) {
    console.error('Error loading config files:', error);
    errorMessage.value = `加载配置文件失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
});

// 保存环境变量
const saveVariable = async (key: string, values: string[]) => {
  if (!selectedFile.value) return;
  
  try {
    const response = await invoke<{code: number, data: boolean}>('save_env_variable', {
      file_path: selectedFile.value.path,
      key,
      values
    });
    
    if (response.code !== 200) {
      errorMessage.value = `保存失败: ${response.message || '未知错误'}`;
    }
  } catch (error) {
    console.error('Error saving variable:', error);
    errorMessage.value = `保存失败: ${error}`;
  }
};

// 删除环境变量
const deleteVariable = async (key: string) => {
  if (!selectedFile.value) return;
  
  try {
    const response = await invoke<{code: number, data: boolean}>('delete_env_variable', {
      file_path: selectedFile.value.path,
      key
    });
    
    if (response.code === 200) {
      // 从列表中移除变量
      selectedFile.value.variables = selectedFile.value.variables.filter(v => v.key !== key);
    } else {
      errorMessage.value = `删除失败: ${response.message || '未知错误'}`;
    }
  } catch (error) {
    console.error('Error deleting variable:', error);
    errorMessage.value = `删除失败: ${error}`;
  }
};

const handleFileSelect = async (file: EnvFile) => {
  try {
    selectedFile.value = file;
    editingVariable.value = null;
    
    // 如果文件存在且已激活，则加载其详细内容
    if (file.is_active) {
      isLoading.value = true;
      const response = await invoke<{code: number, data: EnvVariable[]}>('load_config_file_content', {
        file_path: file.path
      });
      
      if (response.code === 200 && response.data) {
        // 更新文件的变量列表
        file.variables = response.data;
      } else {
        errorMessage.value = `加载文件内容失败: ${response.message || '未知错误'}`;
      }
    }
  } catch (error) {
    console.error('Error loading file content:', error);
    errorMessage.value = `加载文件内容失败: ${error}`;
  } finally {
    isLoading.value = false;
  }
};

const startEditing = (key: string, index: number) => {
  editingVariable.value = { key, index };
  const variable = selectedFile.value?.variables.find(v => v.key === key);
  if (variable) {
    newValue.value = variable.values[index];
  }
};

const saveEdit = async () => {
  if (!editingVariable.value || !selectedFile.value) return;
  
  const { key, index } = editingVariable.value;
  const variable = selectedFile.value.variables.find(v => v.key === key);
  
  if (variable && newValue.value.trim()) {
    // 更新本地数据
    variable.values[index] = newValue.value.trim();
    
    // 保存到文件
    await saveVariable(key, variable.values);
  }
  
  editingVariable.value = null;
  newValue.value = '';
};

const cancelEdit = () => {
  editingVariable.value = null;
  newValue.value = '';
};

const addValueToVariable = async (key: string) => {
  if (!selectedFile.value) return;
  
  const variable = selectedFile.value.variables.find(v => v.key === key);
  if (variable && newValue.value.trim()) {
    // 添加到本地数据
    variable.values.push(newValue.value.trim());
    
    // 保存到文件
    await saveVariable(key, variable.values);
    
    newValue.value = '';
  }
};

const removeValue = async (key: string, index: number) => {
  if (!selectedFile.value) return;
  
  const variable = selectedFile.value.variables.find(v => v.key === key);
  if (variable && variable.values.length > 1) {
    // 从本地数据中移除
    variable.values.splice(index, 1);
    
    // 保存到文件
    await saveVariable(key, variable.values);
  }
};

const addNewVariable = async () => {
  if (!selectedFile.value || !newVariableKey.value.trim() || !newVariableValue.value.trim()) return;
  
  // Check if variable with this key already exists
  const exists = selectedFile.value.variables.some(v => v.key === newVariableKey.value.trim());
  
  if (!exists) {
    const key = newVariableKey.value.trim();
    const value = newVariableValue.value.trim();
    
    // 添加到本地数据
    selectedFile.value.variables.push({
      key,
      values: [value]
    });
    
    // 保存到文件
    await saveVariable(key, [value]);
    
    newVariableKey.value = '';
    newVariableValue.value = '';
  }
};

const handleSaveAll = async () => {
  try {
    if (!selectedFile.value) return;
    
    const response = await invoke('save_all_variables', {
      file_path: selectedFile.value.path,
      variables: selectedFile.value.variables
    });
    
    if (response.code === 200) {
      errorMessage.value = '';
    }
  } catch (error) {
    errorMessage.value = `保存失败: ${error}`;
  }
};



const filteredVariables = computed(() => {
  if (!selectedFile.value) return [];
  if (!searchQuery.value) return selectedFile.value.variables;
  
  const query = searchQuery.value.toLowerCase();
  return selectedFile.value.variables.filter(variable => 
    variable.key.toLowerCase().includes(query) ||
    variable.values.some(value => value.toLowerCase().includes(query))
  );
});
</script>

<template>
  <div class="env-config">
    
    <div v-if="errorMessage" class="error-message">
      <span>{{ errorMessage }}</span>
      <button @click="errorMessage = ''" class="close-btn">×</button>
    </div>
    
    <div class="env-layout">
      <!-- 左侧文件列表 -->
      <div class="file-list">
        <h3>配置文件</h3>
        <div v-if="isLoading" class="loading-indicator">
          <div class="spinner"></div>
          <span>加载中...</span>
        </div>
        <div v-else class="file-cards">
          <div
            v-for="file in envFiles"
            :key="file.id"
            class="file-card"
            :class="{ 'active': selectedFile?.id === file.id }"
            @click="handleFileSelect(file)"
          >
            <div class="file-header">
              <span class="priority">优先级: {{ file.priority }}</span>
              <span class="status" :class="{ 'active': file.is_active }">
                {{ file.is_active ? '已加载' : '未加载' }}
              </span>
            </div>
            <div class="file-body">
              <h3>{{ file.name }}</h3>
              <p class="file-path">{{ file.path }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧环境变量详情 -->
      <div class="env-details" v-if="selectedFile">
        <h2>{{ selectedFile.name }} 的环境变量</h2>
        <div class="search-bar">
          <input
            type="text"
            v-model="searchQuery"
            placeholder="搜索环境变量..."
            class="search-input"
          />
        </div>
        
        <!-- 添加新变量 -->
        <div class="add-variable-form">
          <h3>添加新环境变量</h3>
          <div class="form-row">
            <input 
              type="text" 
              v-model="newVariableKey" 
              placeholder="变量名称" 
              class="form-input"
            />
            <input 
              type="text" 
              v-model="newVariableValue" 
              placeholder="变量值" 
              class="form-input"
            />
            <button @click="addNewVariable" class="action-button add">添加</button>
          </div>
        </div>
        
        <div class="variables-table">
          <div class="table-header">
            <div class="col-key">变量名</div>
            <div class="col-values">变量值</div>
            <div class="col-actions">操作</div>
          </div>
          
          <div
            v-for="variable in filteredVariables"
            :key="variable.key"
            class="variable-row"
          >
            <div class="col-key">{{ variable.key }}</div>
            <div class="col-values">
              <div class="values-container">
                <div 
                  v-for="(value, index) in variable.values" 
                  :key="index"
                  class="value-item"
                >
                  <div 
                    v-if="editingVariable && editingVariable.key === variable.key && editingVariable.index === index"
                    class="edit-value-form"
                  >
                    <input 
                      type="text" 
                      v-model="newValue" 
                      class="edit-input"
                      @keyup.enter="saveEdit"
                    />
                    <div class="edit-actions">
                      <button @click="saveEdit" class="action-button save">保存</button>
                      <button @click="cancelEdit" class="action-button cancel">取消</button>
                    </div>
                  </div>
                  <div v-else class="value-display">
                    <span class="value-text">{{ value }}</span>
                    <div class="value-actions">
                      <button @click="startEditing(variable.key, index)" class="action-button edit">编辑</button>
                      <button 
                        @click="removeValue(variable.key, index)" 
                        class="action-button delete"
                        :disabled="variable.values.length <= 1"
                      >删除</button>
                    </div>
                  </div>
                </div>
                
                <!-- 添加新值 -->
                <div class="add-value-form">
                  <input 
                    type="text" 
                    v-model="newValue" 
                    placeholder="添加新值" 
                    class="add-value-input"
                    @keyup.enter="addValueToVariable(variable.key)"
                  />
                  <button @click="addValueToVariable(variable.key)" class="action-button add">添加</button>
                </div>
              </div>
            </div>
            <div class="col-actions">
              <!-- 可以添加更多操作按钮，如删除整个变量等 -->
            </div>
          </div>
        </div>
      </div>
      <div class="env-details empty" v-else>
        <div class="empty-message">
          <h3>请选择一个配置文件</h3>
          <p>在左侧列表中选择一个环境变量配置文件以查看详情</p>
        </div>
      </div>
    </div>
  </div>
  <div class="action-bar">
    <button class="action-button save" @click="handleSaveAll">保存全部</button>
  </div>
</template>

<style scoped>
.env-config {
  padding: 15px;
  height: 550px;
  width: 700px;
  background-color: #f6f8fa;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  display: flex;
  flex-direction: column;
}



.error-message {
  background-color: #ffebe9;
  border: 1px solid #ff8182;
  border-radius: 6px;
  padding: 10px 15px;
  margin-bottom: 20px;
  color: #cf222e;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.2em;
  cursor: pointer;
  color: #cf222e;
}

.loading-indicator {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 30px 0;
  color: #586069;
}

.spinner {
  width: 30px;
  height: 30px;
  border: 3px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top-color: #24c8db;
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 10px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.env-layout {
  display: grid;
  grid-template-columns: 180px 1fr;
  gap: 12px;
  flex: 1;
  overflow: hidden;
}

.file-list {
  background: #ffffff;
  border-radius: 8px;
  padding: 10px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
  overflow-y: auto;
  height: 100%;
}

.file-cards {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 15px;
}

.file-card {
  background: #f8f9fa;
  border-radius: 6px;
  padding: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
  font-size: 0.85em;
}

.file-card:hover {
  transform: translateX(4px);
  background: #f0f2f5;
}

.file-card.active {
  border-color: #24c8db;
  background: #ffffff;
}

.file-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.priority {
  font-size: 0.9em;
  color: #666;
}

.status {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 0.8em;
  background: #f5f5f5;
  color: #666;
}

.status.active {
  background: #e6f7ff;
  color: #24c8db;
}

.file-body h3 {
  margin: 0;
  font-size: 1.1em;
  color: #333;
}

.file-path {
  margin: 8px 0 0;
  font-size: 0.9em;
  color: #666;
}

.env-details {
  background: #ffffff;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow-y: auto;
}

.env-details.empty {
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: #666;
}

.empty-message h3 {
  margin: 0 0 8px;
  color: #333;
}

.empty-message p {
  margin: 0;
  color: #666;
}

.variables-list {
  margin-top: 20px;
}

.search-bar {
  margin-bottom: 20px;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  font-size: 0.9em;
  outline: none;
}

.search-input:focus {
  border-color: #24c8db;
}

.add-variable-form {
  margin: 15px 0;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 6px;
}

.add-variable-form h3 {
  margin-top: 0;
  margin-bottom: 10px;
  font-size: 1em;
  color: #333;
}

.form-row {
  display: flex;
  gap: 10px;
  align-items: center;
}

.form-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  font-size: 0.9em;
}

.variables-table {
  margin-top: 20px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 120px 1fr 80px;
  background: #f8f9fa;
  padding: 8px 12px;
  font-size: 0.9em;
  border-bottom: 1px solid #e0e0e0;
}

.variable-row {
  display: grid;
  grid-template-columns: 120px 1fr 90px;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid #f0f0f0;
  font-size: 0.9em;
}

.variable-row:last-child {
  border-bottom: none;
}

.col-key, .col-values, .col-actions {
  padding: 12px 15px;
}

.col-key {
  font-weight: 600;
  color: #333;
  border-right: 1px solid #e0e0e0;
  display: flex;
  align-items: center;
}

.values-container {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
}

.value-item {
  margin-bottom: 8px;
}

.value-item:last-child {
  margin-bottom: 0;
}

.value-display {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #f8f9fa;
  border-radius: 4px;
  padding: 6px 10px;
  max-width: 200px;
}

.value-text {
  font-family: monospace;
  color: #333;
}

.value-actions, .edit-actions {
  display: flex;
  gap: 5px;
}

.action-button {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8em;
  cursor: pointer;
  border: none;
  color: white;
}

.action-button.edit {
  background: #1890ff;
}

.action-button.delete {
  background: #ff4d4f;
}

.action-button.delete:disabled {
  background: #d9d9d9;
  cursor: not-allowed;
}

.action-button.save {
  background: #52c41a;
}

.action-button.cancel {
  background: #faad14;
}

.action-button.add {
  background: #722ed1;
}

.edit-value-form {
  display: flex;
  flex-direction: column;
  gap: 8px;
  background: #f0f7ff;
  padding: 8px 12px;
  border-radius: 6px;
}

.edit-input {
  width: 100%;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-family: monospace;
}

.add-value-form {
  display: flex;
  gap: 8px;
  margin-top: 10px;
  align-items: center;
}

.add-value-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-family: monospace;
}
.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 10px 0;
  margin-top: 10px;
  border-top: 1px solid #e0e0e0;
}

.action-bar .action-button {
  padding: 6px 12px;
  font-size: 0.9em;
}
</style>