// ============================================================================
// Chat History Persistence - JavaScript Functions
// ============================================================================

// Get DOM elements
const getChatHistory = () => document.getElementById('chat-history');
const getConversationId = () => window.conversationId || null;
const setConversationId = (id) => { window.conversationId = id; };

// Chat history state
let chatHistoryMetadata = {
    provider: null,
    model: null,
    conversation_id: null,
    chat_mode: false,
    encryption_used: false,
    created_at: null
};

// Default chat history directory
const DEFAULT_CHAT_DIR = 'ApiAi_Chats';

// Get chat history directory path
async function getChatHistoryDir() {
    const { homeDir } = window.__TAURI__.path;
    const home = await homeDir();
    return `${home}/${DEFAULT_CHAT_DIR}`;
}

// Ensure chat history directory exists
async function ensureChatHistoryDir() {
    const { exists, mkdir } = window.__TAURI__.fs;
    const dir = await getChatHistoryDir();

    const dirExists = await exists(dir);
    if (!dirExists) {
        await mkdir(dir, { recursive: true });
    }

    return dir;
}

// Get current chat data
function getCurrentChatData() {
    const chatHistory = getChatHistory();
    const messages = Array.from(chatHistory.querySelectorAll('.message:not(.system):not(.error)'));

    const chatMessages = messages.map(msg => {
        const role = msg.classList.contains('user') ? 'user' : 'assistant';
        const content = msg.querySelector('.content').innerText;
        const timestamp = msg.dataset.timestamp || new Date().toISOString();

        const message = {
            role,
            content,
            timestamp
        };

        // Add provider/model if available (for AI messages)
        if (role === 'assistant' && chatHistoryMetadata.provider) {
            message.provider = chatHistoryMetadata.provider;
            message.model = chatHistoryMetadata.model;
        }

        return message;
    });

    const now = new Date().toISOString();

    return {
        version: '1.0',
        metadata: {
            created_at: chatHistoryMetadata.created_at || now,
            last_modified: now,
            provider: chatHistoryMetadata.provider,
            model: chatHistoryMetadata.model,
            conversation_id: getConversationId(),
            chat_mode: document.getElementById('chat-mode').checked,
            encryption_used: chatHistoryMetadata.encryption_used,
            message_count: chatMessages.length
        },
        messages: chatMessages
    };
}

// Save chat history
async function saveChatHistory() {
    try {
        const { save } = window.__TAURI__.dialog;

        const chatData = getCurrentChatData();

        if (chatData.messages.length === 0) {
            alert('No messages to save!');
            return;
        }

        const timestamp = Date.now();
        const defaultName = `chat-${timestamp}.json`;

        const filePath = await save({
            defaultPath: defaultName,
            filters: [{
                name: 'Chat History',
                extensions: ['json']
            }]
        });

        if (filePath) {
            await invoke('save_chat_history', {
                chatData,
                filePath
            });

            console.log('Chat saved to:', filePath);

            // Visual feedback
            const saveChatBtn = document.getElementById('save-chat');
            const originalText = saveChatBtn.textContent;
            saveChatBtn.textContent = '✅ Saved!';
            setTimeout(() => {
                saveChatBtn.textContent = originalText;
            }, 2000);
        }
    } catch (error) {
        console.error('Failed to save chat:', error);
        alert('Failed to save chat: ' + error);
    }
}

// Load chat history
async function loadChatHistory() {
    try {
        const { open } = window.__TAURI__.dialog;

        const filePath = await open({
            filters: [{
                name: 'Chat History',
                extensions: ['json']
            }]
        });

        if (filePath) {
            const chatData = await invoke('load_chat_history', { filePath });
            restoreChatFromData(chatData);

            console.log('Chat loaded from:', filePath);

            // Visual feedback
            const loadChatBtn = document.getElementById('load-chat');
            const originalText = loadChatBtn.textContent;
            loadChatBtn.textContent = '✅ Loaded!';
            setTimeout(() => {
                loadChatBtn.textContent = originalText;
            }, 2000);
        }
    } catch (error) {
        console.error('Failed to load chat:', error);
        alert('Failed to load chat: ' + error);
    }
}

// Restore chat from data
function restoreChatFromData(chatData) {
    const chatHistory = getChatHistory();

    // Clear current chat
    chatHistory.innerHTML = '';

    // Restore metadata
    chatHistoryMetadata = {
        provider: chatData.metadata.provider,
        model: chatData.metadata.model,
        conversation_id: chatData.metadata.conversation_id,
        chat_mode: chatData.metadata.chat_mode,
        encryption_used: chatData.metadata.encryption_used,
        created_at: chatData.metadata.created_at
    };

    // Restore conversation ID
    setConversationId(chatData.metadata.conversation_id);

    // Restore chat mode
    document.getElementById('chat-mode').checked = chatData.metadata.chat_mode;

    // Restore messages
    chatData.messages.forEach(msg => {
        if (window.appendMessage) {
            window.appendMessage(msg.content, msg.role, {
                provider: msg.provider,
                model: msg.model,
                timestamp: msg.timestamp
            });
        }
    });

    // Update provider info
    if (chatData.metadata.provider && chatData.metadata.model && window.currentProviderInfo) {
        window.currentProviderInfo.provider = chatData.metadata.provider;
        window.currentProviderInfo.model = chatData.metadata.model;
        if (window.updateProviderInfo) {
            window.updateProviderInfo();
        }
    }

    if (window.scrollToBottom) {
        window.scrollToBottom();
    }
}

// Show chat library
async function showChatLibrary() {
    try {
        const dir = await ensureChatHistoryDir();
        const chats = await invoke('list_saved_chats', { directory: dir });

        const libraryModal = document.getElementById('chat-library-modal');
        const libraryList = document.getElementById('library-list');

        // Clear list
        libraryList.innerHTML = '';

        if (chats.length === 0) {
            libraryList.innerHTML = `
        <div class="library-empty">
          <div class="library-empty-icon">📚</div>
          <div class="library-empty-text">No saved chats yet</div>
          <div class="library-empty-hint">Save your first chat to see it here!</div>
        </div>
      `;
        } else {
            chats.forEach(chat => {
                const item = createLibraryItem(chat);
                libraryList.appendChild(item);
            });
        }

        libraryModal.classList.remove('hidden');
    } catch (error) {
        console.error('Failed to load chat library:', error);
        alert('Failed to load chat library: ' + error);
    }
}

// Create library item element
function createLibraryItem(chat) {
    const div = document.createElement('div');
    div.className = 'library-item';
    div.dataset.path = chat.path;

    const date = new Date(chat.metadata.last_modified);
    const dateStr = date.toLocaleString();

    const provider = chat.metadata.provider || 'Unknown';
    const model = chat.metadata.model || 'Unknown';
    const messageCount = chat.metadata.message_count || 0;

    div.innerHTML = `
    <div class="library-item-header">
      <div class="library-item-title">${chat.filename}</div>
      <div class="library-item-date">${dateStr}</div>
    </div>
    <div class="library-item-preview">${chat.preview}</div>
    <div class="library-item-meta">
      <span>🤖 ${provider}</span>
      <span>📝 ${model}</span>
      <span>💬 ${messageCount} messages</span>
    </div>
  `;

    div.addEventListener('click', async () => {
        try {
            const chatData = await invoke('load_chat_history', { filePath: chat.path });
            restoreChatFromData(chatData);
            document.getElementById('chat-library-modal').classList.add('hidden');
        } catch (error) {
            console.error('Failed to load chat:', error);
            alert('Failed to load chat: ' + error);
        }
    });

    return div;
}

// Import chat from TXT/MD
async function importChat() {
    try {
        const { open } = window.__TAURI__.dialog;

        const filePath = await open({
            filters: [{
                name: 'Text Files',
                extensions: ['txt', 'md']
            }]
        });

        if (filePath) {
            const chatData = await invoke('import_text_chat', { filePath });
            restoreChatFromData(chatData);

            console.log('Chat imported from:', filePath);

            // Visual feedback
            const importChatBtn = document.getElementById('import-chat');
            const originalText = importChatBtn.textContent;
            importChatBtn.textContent = '✅ Imported!';
            setTimeout(() => {
                importChatBtn.textContent = originalText;
            }, 2000);
        }
    } catch (error) {
        console.error('Failed to import chat:', error);
        alert('Failed to import chat: ' + error);
    }
}

// Export to Markdown
async function exportToMarkdown() {
    try {
        const { save } = window.__TAURI__.dialog;
        const { writeTextFile } = window.__TAURI__.fs;

        const chatHistory = getChatHistory();
        const messages = Array.from(chatHistory.querySelectorAll('.message:not(.system):not(.error)'));

        if (messages.length === 0) {
            alert('No messages to export!');
            return;
        }

        let markdown = '# ApiAi Chat Export\n\n';
        markdown += `**Date:** ${new Date().toLocaleString()}\n\n`;

        if (chatHistoryMetadata.provider) {
            markdown += `**Provider:** ${chatHistoryMetadata.provider}\n`;
        }
        if (chatHistoryMetadata.model) {
            markdown += `**Model:** ${chatHistoryMetadata.model}\n`;
        }
        const convId = getConversationId();
        if (convId) {
            markdown += `**Conversation ID:** ${convId}\n`;
        }

        markdown += '\n---\n\n';

        messages.forEach((msg, index) => {
            const role = msg.classList.contains('user') ? 'User' : 'AI';
            const content = msg.querySelector('.content').innerText;

            markdown += `## ${role}\n\n`;
            markdown += `${content}\n\n`;

            if (index < messages.length - 1) {
                markdown += '---\n\n';
            }
        });

        const timestamp = Date.now();
        const filePath = await save({
            defaultPath: `apiai-chat-${timestamp}.md`,
            filters: [{
                name: 'Markdown Files',
                extensions: ['md']
            }]
        });

        if (filePath) {
            await writeTextFile(filePath, markdown);
            console.log('Exported to Markdown:', filePath);

            // Visual feedback
            const exportMdBtn = document.getElementById('export-md');
            const originalText = exportMdBtn.textContent;
            exportMdBtn.textContent = '✅ MD';
            setTimeout(() => {
                exportMdBtn.textContent = originalText;
            }, 2000);
        }
    } catch (error) {
        console.error('Markdown export failed:', error);
        alert('Markdown export failed: ' + error);
    }
}

// Library search and filter
function setupLibrarySearch() {
    const searchInput = document.getElementById('library-search-input');
    const filterSelect = document.getElementById('library-filter');

    const filterLibrary = () => {
        const searchTerm = searchInput.value.toLowerCase();
        const filterValue = filterSelect.value;

        const items = document.querySelectorAll('.library-item');

        items.forEach(item => {
            const preview = item.querySelector('.library-item-preview').textContent.toLowerCase();
            const meta = item.querySelector('.library-item-meta').textContent.toLowerCase();

            const matchesSearch = preview.includes(searchTerm) || meta.includes(searchTerm);
            const matchesFilter = filterValue === 'all' || meta.includes(filterValue.toLowerCase());

            if (matchesSearch && matchesFilter) {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
    };

    searchInput.addEventListener('input', filterLibrary);
    filterSelect.addEventListener('change', filterLibrary);
}

// Open chat history folder
async function openChatHistoryFolder() {
    try {
        const { open } = window.__TAURI__.shell;
        const dir = await ensureChatHistoryDir();
        await open(dir);
    } catch (error) {
        console.error('Failed to open folder:', error);
        alert('Failed to open folder: ' + error);
    }
}

// Extend appendMessage from main.js to add timestamp tracking
// We'll use MutationObserver to track new messages
function setupMessageTracking() {
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            mutation.addedNodes.forEach((node) => {
                if (node.classList && node.classList.contains('message')) {
                    // Add timestamp if not already present
                    if (!node.dataset.timestamp) {
                        node.dataset.timestamp = new Date().toISOString();
                    }
                }
            });
        });
    });

    const chatHistory = getChatHistory();
    observer.observe(chatHistory, { childList: true });
}

// Track metadata from AI responses
function trackAIMetadata(provider, model) {
    chatHistoryMetadata.provider = provider;
    chatHistoryMetadata.model = model;
    chatHistoryMetadata.encryption_used = document.getElementById('use-encryption').checked;

    if (!chatHistoryMetadata.created_at) {
        chatHistoryMetadata.created_at = new Date().toISOString();
    }
}

// Export tracking function for use in main.js
window.trackAIMetadata = trackAIMetadata;

// Event Listeners for new buttons
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('save-chat').addEventListener('click', saveChatHistory);
    document.getElementById('load-chat').addEventListener('click', loadChatHistory);
    document.getElementById('chat-library').addEventListener('click', showChatLibrary);
    document.getElementById('import-chat').addEventListener('click', importChat);
    document.getElementById('export-md').addEventListener('click', exportToMarkdown);

    // Library modal controls
    document.getElementById('library-close').addEventListener('click', () => {
        document.getElementById('chat-library-modal').classList.add('hidden');
    });

    document.getElementById('library-refresh').addEventListener('click', showChatLibrary);
    document.getElementById('library-open-folder').addEventListener('click', openChatHistoryFolder);

    // Setup library search
    setupLibrarySearch();

    // Setup message tracking
    setupMessageTracking();

    console.log('Chat history persistence initialized');
});

