export interface ConfigFile {
    path: string;
    id: number;  // u8 转换为 TypeScript 的 number
}

export interface EnvVariableResponse {
    index: number;  // u64 转换为 number (注意大数情况)
    key: string;
    value: string[];  // Vec<String> 转换为 string[]
}

export interface EnvVariableRequest {
    index: number;
    key: string;
    value: string[];
}

export interface ApiResponse<T> {
    code: number;      // u16 → number
    message: string;   // String → string
    data: T | null;    // Option<T> → T | null
    error: string | null; // Option<String> → string | null
}
