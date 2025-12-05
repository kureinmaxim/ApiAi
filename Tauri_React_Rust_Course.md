# 🚀 Курс: Создание AI-чата на Tauri + React + Rust (2025)

Этот курс из 10 уроков проведет вас от основ React до создания полноценного десктопного приложения на Tauri, используя реальные примеры из проекта **ApiAi**. Мы разберем, как переписать текущий Vanilla JS фронтенд на современный React + TypeScript стек, сохранив мощный Rust бэкенд.

---

## 🟩 Неделя 1: Фундамент (React + TypeScript)

### Урок 1: Компонентный подход (UI)
**Цель:** Разбить монолитный интерфейс `index.html` на переиспользуемые React-компоненты.

В текущем проекте `ApiAi` весь UI находится в одном HTML файле. В React мы разделим его на логические части.

**Пример из проекта:**
Вместо ручного создания HTML строк в `main.js` (`chatHistory.innerHTML = ...`), мы создадим компонент `ChatMessage`.

```tsx
// src/components/ChatMessage.tsx
import React from 'react';

interface ChatMessageProps {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp?: string;
}

export const ChatMessage: React.FC<ChatMessageProps> = ({ role, content, timestamp }) => {
  const isUser = role === 'user';
  return (
    <div className={`message ${role} ${isUser ? 'ml-auto bg-blue-500' : 'bg-gray-700'}`}>
      <div className="content">{content}</div>
      {timestamp && <div className="timestamp text-xs opacity-50">{timestamp}</div>}
    </div>
  );
};
```

**Задание:** Создайте компоненты `InputArea` (поле ввода и кнопка отправки) и `Sidebar` (история чатов), основываясь на структуре `index.html`.

### Урок 2: State и Hooks (Управление состоянием)
**Цель:** Заменить глобальные переменные (`let conversationId`, `let isProcessing`) на React Hooks.

В `main.js` состояние разбросано по глобальным переменным. В React мы используем `useState`.

```tsx
// src/App.tsx
import { useState } from 'react';
import { ChatMessage } from './components/ChatMessage';

export default function App() {
  // Вместо let conversationId = null;
  const [conversationId, setConversationId] = useState<string | null>(null);
  
  // Вместо const chatHistory = ...
  const [messages, setMessages] = useState<Array<{role: string, content: string}>>([]);
  
  // Вместо let isProcessing = false;
  const [isProcessing, setIsProcessing] = useState(false);

  const handleSend = async (text: string) => {
    setIsProcessing(true);
    // Добавляем сообщение пользователя
    setMessages(prev => [...prev, { role: 'user', content: text }]);
    
    // Логика отправки (см. Урок 5)
    
    setIsProcessing(false);
  };

  return (
    <div className="app-container">
      <div className="chat-history">
        {messages.map((msg, idx) => (
          <ChatMessage key={idx} {...msg} />
        ))}
      </div>
      {/* ... InputArea ... */}
    </div>
  );
}
```

### Урок 3: TypeScript и Типизация данных
**Цель:** Синхронизировать типы данных между Rust и Frontend.

В `src-tauri/src/lib.rs` у нас есть четкие структуры. Давайте опишем их в TypeScript, чтобы избежать ошибок "undefined is not a function".

**Rust (lib.rs):**
```rust
#[derive(Clone, Serialize, Deserialize)]
struct AppConfig {
    security: SecurityConfig,
    api_keys: ApiKeysConfig,
    ui: UiConfig,
}
```

**TypeScript (src/types.ts):**
```typescript
export interface SecurityConfig {
  pin_code: string;
  require_pin: boolean;
}

export interface ApiKeysConfig {
  anthropic: string;
  openai: string;
  telegram_url: string;
  telegram_key: string;
  telegram_enc_key: string;
  telegram_use_encryption: boolean;
}

export interface UiConfig {
  theme: string;
  window_width: number | null;
  window_height: number | null;
}

export interface AppConfig {
  security: SecurityConfig;
  api_keys: ApiKeysConfig;
  ui: UiConfig;
}

export interface SearchResponse {
  text: string;
  provider: string;
  model?: string;
  conversation_id?: string;
  request_id?: string;
}
```

---

## 🟦 Неделя 2: Основы Tauri

### Урок 4: Архитектура Tauri приложения
**Цель:** Понять, как Rust и WebView живут вместе.

*   **Backend (Rust):** `src-tauri` — это "мозг". Здесь живет логика, работа с файлами, сетью и безопасностью.
*   **Frontend (WebView):** `src` — это "лицо". Обычный веб-сайт, который отображается в окне ОС.

В нашем проекте:
*   `src-tauri/tauri.conf.json` — паспорт приложения (название, версия, права доступа).
*   `src-tauri/src/lib.rs` — точка входа для команд.

**Практика:** Изучите `tauri.conf.json`. Обратите внимание на раздел `allowlist` (или `capabilities` в v2), который разрешает фронтенду доступ к файловой системе (`fs`) и диалогам (`dialog`).

### Урок 5: Мост между мирами (Invoke)
**Цель:** Научиться вызывать Rust функции из React.

В `main.js` используется:
```javascript
const { invoke } = window.__TAURI__.core;
await invoke('perform_search', { ... });
```

В React + TS мы сделаем это типобезопасным:

```typescript
// src/api/tauri.ts
import { invoke } from '@tauri-apps/api/core';
import { SearchResponse, AppConfig } from '../types';

export const api = {
  performSearch: async (
    query: string, 
    provider: string, 
    apiKey: string
  ): Promise<SearchResponse> => {
    return await invoke('perform_search', {
      query,
      provider,
      apiKey,
      // ... остальные параметры
    });
  },

  getConfig: async (): Promise<AppConfig> => {
    return await invoke('get_config');
  }
};
```

Теперь в компоненте:
```tsx
const config = await api.getConfig(); // config имеет тип AppConfig!
```

---

## 🟫 Неделя 3: Rust Backend

### Урок 6: Структура Rust бэкенда и State
**Цель:** Разобраться, как Rust хранит состояние приложения.

В `lib.rs` мы видим:
```rust
struct AppState {
    config: Mutex<AppConfig>,
}
```
`Mutex` нужен для безопасного доступа к данным из разных потоков (ведь команд может быть много одновременно).

**Команда `get_config`:**
```rust
#[tauri::command]
fn get_config(state: State<AppState>) -> AppConfig {
    let config = state.config.lock().unwrap(); // Блокируем мьютекс
    config.clone() // Возвращаем копию данных
}
```
**Задание:** Добавьте в `AppConfig` новое поле `language: String` и обновите методы `default()` и `save_config`.

### Урок 7: Асинхронность и `perform_search`
**Цель:** Понять, как работают долгие операции, чтобы не "вешать" интерфейс.

Функция `perform_search` помечена как `async`. Это критично для сетевых запросов.

```rust
#[tauri::command]
async fn perform_search(
    query: String,
    // ... аргументы
) -> Result<SearchResponse, String> {
    // ... выбор клиента ...
    
    // await не блокирует UI поток!
    match client.search(&query).await {
        Ok(result) => Ok(SearchResponse { ... }),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
```
Tauri автоматически запускает `async` команды в отдельном потоке. Если бы мы убрали `async` и использовали блокирующий `reqwest::blocking`, интерфейс приложения замирал бы на время запроса.

---

## 🟧 Неделя 4: Интеграция и Данные

### Урок 8: Сериализация и Serde
**Цель:** Как Rust объекты превращаются в JSON для JS.

Магия происходит благодаря макросам `#[derive(Serialize, Deserialize)]`.
В `lib.rs`:
```rust
#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    text: String,
    // ...
}
```
Библиотека `serde` превращает эту структуру в JSON строку, которую Tauri отправляет в WebView.
Если вы забудете `Serialize`, код не скомпилируется при попытке вернуть структуру из команды.

**Практика:** Попробуйте вернуть из команды структуру без `Serialize` и посмотрите на ошибку компилятора.

### Урок 9: Работа с файлами и персистентность
**Цель:** Сохранение истории чатов на диск.

В проекте есть функции `save_chat_history` и `load_chat_history`.
Они используют `std::fs` для записи JSON файлов.

```rust
#[tauri::command]
fn save_chat_history(chat_data: ChatHistory, file_path: String) -> Result<(), String> {
    // 1. Сериализуем в строку
    let content = serde_json::to_string_pretty(&chat_data)
        .map_err(|e| ...)?;
    
    // 2. Пишем в файл
    fs::write(&file_path, content)
        .map_err(|e| ...)?;
    
    Ok(())
}
```
На фронтенде мы используем диалоги сохранения:
```typescript
import { save } from '@tauri-apps/plugin-dialog';

const saveChat = async () => {
  const path = await save({
    filters: [{ name: 'JSON', extensions: ['json'] }]
  });
  if (path) {
    await invoke('save_chat_history', { chatData: currentChat, filePath: path });
  }
};
```

---

## 🟨 Неделя 5: Продвинутые техники

### Урок 10: Отмена запросов и Events
**Цель:** Реализовать кнопку "Стоп", как в `main.js`.

В `main.js` есть `abortRequest`. В Rust это реализовано через отдельную команду `cancel_request`.

**Схема работы:**
1. Фронтенд генерирует `requestId` (UUID).
2. Отправляет запрос `perform_search` с этим ID.
3. Если пользователь жмет "Стоп", фронтенд вызывает `cancel_request(requestId)`.
4. Бэкенд отправляет сигнал отмены на сервер (или убивает локальный процесс).

**Важно:** Для локальных долгих задач (не HTTP) лучше использовать `tauri::async_runtime::spawn` и `AtomicBool` флаги или каналы `tokio::sync::mpsc`.

---

## 🏁 Заключение

Вы прошли путь от простого скрипта до архитектурно правильного приложения.
**Что дальше?**
1. Соберите приложение: `npm run tauri build`.
2. Найдите установщик в `src-tauri/target/release/bundle`.
3. Поделитесь своим AI-чатом с миром!
