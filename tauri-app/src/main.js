const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

let conversationId = null;
const CORRECT_PIN = "1234"; // From config_qt.json

// DOM Elements - PIN Modal
const pinModal = document.getElementById('pin-modal');
const pinInput = document.getElementById('pin-input');
const pinError = document.getElementById('pin-error');
const pinCancel = document.getElementById('pin-cancel');
const mainApp = document.getElementById('main-app');

// DOM Elements - Main App
const chatHistory = document.getElementById('chat-history');
const promptInput = document.getElementById('prompt-input');
const sendBtn = document.getElementById('send-btn');
const clearBtn = document.getElementById('clear-chat');
const lockIndicator = document.getElementById('lock-indicator');
const developerName = document.getElementById('developer-name');
const providerSelect = document.getElementById('provider');
const telegramSettings = document.getElementById('telegram-settings');
const useEncryption = document.getElementById('use-encryption');
const encryptionKeyInput = document.getElementById('encryption-key');
const windowSizeDisplay = document.getElementById('window-size');

// Protected fields
// Protected fields - will be re-queried or we can use a getter if needed, 
// but querySelectorAll returns a static NodeList. 
// Since we added the class in HTML, the initial query will catch it.
const protectedFields = document.querySelectorAll('.protected-field');

// State
let isProcessing = false;
let settingsUnlocked = false;

// Initialize
async function init() {
  // App opens directly, settings are locked by default
  updateWindowSize();

  try {
    // Load config from backend
    const config = await invoke('get_config');
    console.log('Loaded config:', config);

    // Populate fields
    if (config.api_keys) {
      // Store keys in data attributes or global state if needed, 
      // but for now we'll populate the fields based on selected provider

      // We need to know which provider is selected to show the right key
      // For now, let's just default to populating based on current selection logic
      // or store them to populate when provider changes

      window.appConfig = config; // Store globally for access

      // Populate Telegram settings
      if (config.api_keys.telegram_url) {
        // Just set the value as is. The sendMessage logic handles full URLs correctly.
        document.getElementById('telegram-url').value = config.api_keys.telegram_url;

        // Try to extract port for the port field just for display/convenience
        try {
          if (config.api_keys.telegram_url.includes('://')) {
            const url = new URL(config.api_keys.telegram_url);
            if (url.port) {
              document.getElementById('telegram-port').value = url.port;
            }
          }
        } catch (e) {
          // Ignore parsing errors
        }
      }

      if (config.api_keys.telegram_enc_key) {
        encryptionKeyInput.value = config.api_keys.telegram_enc_key;
      }

      if (config.api_keys.telegram_use_encryption !== undefined) {
        useEncryption.checked = config.api_keys.telegram_use_encryption;
        if (useEncryption.checked) {
          encryptionKeyInput.classList.remove('hidden');
        }
      }

      // Trigger provider change to populate API key for default provider
      updateApiKeyField();
    }

  } catch (e) {
    console.error('Failed to load config:', e);
  }

  promptInput.focus();
}

function updateApiKeyField() {
  const provider = providerSelect.value;
  const apiKeyInput = document.getElementById('api-key');

  if (window.appConfig && window.appConfig.api_keys) {
    if (provider === 'telegram') {
      apiKeyInput.value = window.appConfig.api_keys.telegram_key || '';
    } else if (provider === 'anthropic') {
      apiKeyInput.value = window.appConfig.api_keys.anthropic || '';
    } else if (provider === 'openai') {
      apiKeyInput.value = window.appConfig.api_keys.openai || '';
    }
  }
}

// Lock/Unlock Settings
function showPinModal() {
  pinModal.classList.remove('hidden');
  pinInput.value = '';
  pinError.classList.add('hidden');
  pinInput.focus();
}

function hidePinModal() {
  pinModal.classList.add('hidden');
}

function unlockSettings() {
  settingsUnlocked = true;
  protectedFields.forEach(field => {
    field.disabled = false;
    field.classList.remove('protected-field');
  });
  lockIndicator.classList.remove('locked');
  lockIndicator.classList.add('unlocked');
  lockIndicator.textContent = '🔓';
  lockIndicator.title = 'Settings Unlocked - Click to lock';
  hidePinModal();
}

function lockSettings() {
  settingsUnlocked = false;
  protectedFields.forEach(field => {
    field.disabled = true;
    field.classList.add('protected-field');
  });
  lockIndicator.classList.remove('unlocked');
  lockIndicator.classList.add('locked');
  lockIndicator.textContent = '🔒';
  lockIndicator.title = 'Settings Locked';
}

// PIN Authentication
pinInput.addEventListener('input', (e) => {
  const value = e.target.value;
  if (value.length === 4) {
    checkPin(value);
  }
  // Hide error when typing
  if (value.length > 0) {
    pinError.classList.add('hidden');
  }
});

pinInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    checkPin(e.target.value);
  } else if (e.key === 'Escape') {
    hidePinModal();
  }
});

function checkPin(pin) {
  if (pin === CORRECT_PIN) {
    unlockSettings();
  } else {
    pinError.classList.remove('hidden');
    pinInput.value = '';
    pinInput.classList.add('shake');
    setTimeout(() => {
      pinInput.classList.remove('shake');
    }, 400);
  }
}

// Lock indicator click
lockIndicator.addEventListener('click', () => {
  if (settingsUnlocked) {
    lockSettings();
  } else {
    showPinModal();
  }
});

// Developer name double-click
let clickCount = 0;
let clickTimer = null;

developerName.addEventListener('click', () => {
  clickCount++;

  if (clickCount === 1) {
    clickTimer = setTimeout(() => {
      clickCount = 0;
    }, 300);
  } else if (clickCount === 2) {
    clearTimeout(clickTimer);
    clickCount = 0;
    if (!settingsUnlocked) {
      showPinModal();
    }
  }
});

// PIN cancel button
pinCancel.addEventListener('click', hidePinModal);

// Window Size
async function updateWindowSize() {
  try {
    const window = getCurrentWindow();
    const size = await window.innerSize();
    windowSizeDisplay.textContent = `${size.width}×${size.height}`;
  } catch (error) {
    windowSizeDisplay.textContent = '-';
  }
}

// Listen for window resize
let resizeTimeout;
window.addEventListener('resize', () => {
  clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(updateWindowSize, 100);
});

// Event Listeners
providerSelect.addEventListener('change', () => {
  if (providerSelect.value === 'telegram') {
    telegramSettings.style.display = 'flex';
  } else {
    telegramSettings.style.display = 'none';
  }
  updateApiKeyField();
});

useEncryption.addEventListener('change', () => {
  if (useEncryption.checked) {
    encryptionKeyInput.classList.remove('hidden');
  } else {
    encryptionKeyInput.classList.add('hidden');
  }
});

// Show Keys Toggle
const showKeysCheckbox = document.getElementById('show-keys');
const apiKeyInput = document.getElementById('api-key');

showKeysCheckbox.addEventListener('change', () => {
  const type = showKeysCheckbox.checked ? 'text' : 'password';
  apiKeyInput.type = type;
  encryptionKeyInput.type = type;
});

// Real-time config updates
apiKeyInput.addEventListener('input', () => {
  if (!window.appConfig) return;
  const provider = providerSelect.value;

  if (provider === 'telegram') {
    window.appConfig.api_keys.telegram_key = apiKeyInput.value;
  } else if (provider === 'anthropic') {
    window.appConfig.api_keys.anthropic = apiKeyInput.value;
  } else if (provider === 'openai') {
    window.appConfig.api_keys.openai = apiKeyInput.value;
  }
});

document.getElementById('telegram-url').addEventListener('input', (e) => {
  if (window.appConfig) window.appConfig.api_keys.telegram_url = e.target.value;
});

encryptionKeyInput.addEventListener('input', (e) => {
  if (window.appConfig) window.appConfig.api_keys.telegram_enc_key = e.target.value;
});

useEncryption.addEventListener('change', (e) => {
  if (window.appConfig) window.appConfig.api_keys.telegram_use_encryption = e.target.checked;
});

sendBtn.addEventListener('click', sendMessage);

promptInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
    sendMessage();
  }
});

// Save Settings
const saveSettingsBtn = document.getElementById('save-settings');

saveSettingsBtn.addEventListener('click', async () => {
  if (!window.appConfig) return;

  // Update config object from UI
  const newConfig = { ...window.appConfig };

  // Update API keys based on current fields
  // Note: This is a bit tricky because we only show one key field at a time.
  // Ideally, we should store all keys in memory and update them as the user switches providers.
  // For now, let's update the currently visible key.

  const provider = providerSelect.value;
  const currentKey = document.getElementById('api-key').value;

  if (provider === 'telegram') {
    newConfig.api_keys.telegram_key = currentKey;
    newConfig.api_keys.telegram_url = document.getElementById('telegram-url').value;
    newConfig.api_keys.telegram_enc_key = encryptionKeyInput.value;
    newConfig.api_keys.telegram_use_encryption = useEncryption.checked;
  } else if (provider === 'anthropic') {
    newConfig.api_keys.anthropic = currentKey;
  } else if (provider === 'openai') {
    newConfig.api_keys.openai = currentKey;
  }

  try {
    await invoke('save_config', { newConfig });

    // Visual feedback
    const originalText = saveSettingsBtn.textContent;
    saveSettingsBtn.textContent = '✅ Saved!';
    saveSettingsBtn.classList.add('success');

    setTimeout(() => {
      saveSettingsBtn.textContent = originalText;
      saveSettingsBtn.classList.remove('success');
    }, 2000);

    // Update global config
    window.appConfig = newConfig;

  } catch (e) {
    console.error('Failed to save settings:', e);
    alert('Failed to save settings: ' + e);
  }
});

clearBtn.addEventListener('click', () => {
  conversationId = null;
  chatHistory.innerHTML = '<div class="message system"><div class="content">Chat cleared. Context reset.</div></div>';
});

// Echo button - inserts text into prompt field
const echoBtn = document.getElementById('echo-btn');
echoBtn.addEventListener('click', () => {
  const currentText = promptInput.value.trim();
  if (currentText) {
    // If there's text, insert it again (echo current input)
    promptInput.value = currentText;
  } else {
    // If empty, insert default echo text
    promptInput.value = 'Echo 123456789';
  }
  promptInput.focus();
});


async function sendMessage() {
  const query = promptInput.value.trim();
  if (!query || isProcessing) return;

  // UI Updates
  appendMessage(query, 'user');
  promptInput.value = '';
  isProcessing = true;
  sendBtn.disabled = true;
  sendBtn.textContent = '...';

  // Gather Settings
  const provider = providerSelect.value;
  const apiKey = document.getElementById('api-key').value;
  const chatMode = document.getElementById('chat-mode').checked;

  // Telegram specific
  let telegramUrl = null;
  let encryptionKey = null;
  let useEnc = false;

  if (provider === 'telegram') {
    const host = document.getElementById('telegram-url').value;
    const port = document.getElementById('telegram-port').value;

    if (host.includes('://')) {
      telegramUrl = host;
    } else {
      telegramUrl = `http://${host}:${port}/ai_query`;
    }

    useEnc = useEncryption.checked;
    if (useEnc) {
      encryptionKey = encryptionKeyInput.value;
    }
  }

  try {
    const response = await invoke('perform_search', {
      query,
      provider,
      apiKey,
      telegramUrl,
      encryptionKey,
      useEncryption: useEnc,
      chatMode,
      conversationId
    });

    appendMessage(response.text, 'ai');

    if (response.conversation_id) {
      conversationId = response.conversation_id;
    }

  } catch (error) {
    appendMessage(`Error: ${error}`, 'error');
  } finally {
    isProcessing = false;
    sendBtn.disabled = false;
    sendBtn.textContent = 'Send 🚀';
    scrollToBottom();
  }
}

function appendMessage(text, type) {
  const div = document.createElement('div');
  div.className = `message ${type}`;

  // Simple markdown-like parsing for bold text
  const formattedText = text.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\n/g, '<br>');

  div.innerHTML = `<div class="content">${formattedText}</div>`;
  chatHistory.appendChild(div);
  scrollToBottom();
}

function scrollToBottom() {
  chatHistory.scrollTop = chatHistory.scrollHeight;
}

// Initialize on load
init();

// Export functionality
const exportTxtBtn = document.getElementById('export-txt');
const exportPdfBtn = document.getElementById('export-pdf');
const exportDocxBtn = document.getElementById('export-docx');

// Helper function to get chat text
function getChatText() {
  const messages = Array.from(chatHistory.querySelectorAll('.message:not(.system):not(.error)'));
  let text = 'ApiAi Chat Export\n';
  text += '='.repeat(50) + '\n\n';

  messages.forEach((msg, index) => {
    const role = msg.classList.contains('user') ? 'User' : 'AI';
    const content = msg.querySelector('.content').innerText;
    text += `${role}:\n${content}\n\n`;
    text += '-'.repeat(50) + '\n\n';
  });

  text += `\nExported: ${new Date().toLocaleString()}`;
  return text;
}

// Export as TXT using Tauri save dialog
exportTxtBtn.addEventListener('click', async () => {
  try {
    const { save } = window.__TAURI__.dialog;
    const { writeTextFile } = window.__TAURI__.fs;

    const text = getChatText();
    const timestamp = Date.now();

    const filePath = await save({
      defaultPath: `apiai-chat-${timestamp}.txt`,
      filters: [{
        name: 'Text Files',
        extensions: ['txt']
      }]
    });

    if (filePath) {
      await writeTextFile(filePath, text);
      console.log('Exported to:', filePath);
      // Visual feedback
      exportTxtBtn.textContent = '✅ TXT';
      setTimeout(() => {
        exportTxtBtn.textContent = '📄 TXT';
      }, 2000);
    }
  } catch (error) {
    console.error('Export failed:', error);
    alert('Export failed: ' + error);
  }
});

// Export as PDF (opens print dialog)
exportPdfBtn.addEventListener('click', async () => {
  try {
    const { message } = window.__TAURI__.dialog;

    // Create a temporary print-friendly window content
    const text = getChatText();
    const htmlContent = `
      <!DOCTYPE html>
      <html>
      <head>
        <title>ApiAi Chat Export</title>
        <style>
          body {
            font-family: Arial, sans-serif;
            padding: 20px;
            max-width: 800px;
            margin: 0 auto;
            color: #333;
          }
          h1 {
            color: #667eea;
            border-bottom: 3px solid #667eea;
            padding-bottom: 10px;
          }
          .message {
            margin: 20px 0;
            padding: 15px;
            border-left: 4px solid #667eea;
            background: #f8f9fa;
          }
          .message-header {
            font-weight: bold;
            color: #667eea;
            margin-bottom: 8px;
          }
          .footer {
            margin-top: 40px;
            padding-top: 20px;
            border-top: 2px solid #ddd;
            color: #666;
            font-size: 0.9em;
          }
          @media print {
            body { padding: 10px; }
            .no-print { display: none; }
          }
        </style>
      </head>
      <body>
        <h1>ApiAi Chat Export</h1>
        <div class="content">
          <pre style="white-space: pre-wrap; font-family: inherit;">${text}</pre>
        </div>
      </body>
      </html>
    `;

    await message('To export as PDF:\n\n1. The print dialog will open\n2. Select "Save as PDF" or "Microsoft Print to PDF"\n3. Choose where to save the file\n\nClick OK to continue.', { title: 'PDF Export', type: 'info' });

    window.print();

  } catch (error) {
    console.error('PDF export failed:', error);
    alert('PDF export failed: ' + error);
  }
});

// Export as DOCX/RTF using Tauri save dialog
exportDocxBtn.addEventListener('click', async () => {
  try {
    const { save } = window.__TAURI__.dialog;
    const { writeTextFile } = window.__TAURI__.fs;

    const text = getChatText();
    const timestamp = Date.now();

    // Create RTF format for better compatibility
    let rtfContent = '{\\rtf1\\ansi\\deff0\n';
    rtfContent += '{\\fonttbl{\\f0\\fnil\\fcharset0 Arial;}}\n';
    rtfContent += '{\\colortbl;\\red102\\green126\\blue234;}\n';
    rtfContent += '\\f0\\fs24\n';
    rtfContent += '{\\b\\fs32\\cf1 ApiAi Chat Export}\\par\\par\n';

    const messages = Array.from(chatHistory.querySelectorAll('.message:not(.system):not(.error)'));
    messages.forEach((msg) => {
      const role = msg.classList.contains('user') ? 'User' : 'AI';
      const content = msg.querySelector('.content').innerText;
      rtfContent += `{\\b ${role}:}\\par\n`;
      rtfContent += content.replace(/\n/g, '\\par\n') + '\\par\\par\n';
      rtfContent += '--------------------\\par\\par\n';
    });

    rtfContent += `\\par Exported: ${new Date().toLocaleString()}\\par\n`;
    rtfContent += '}';

    const filePath = await save({
      defaultPath: `apiai-chat-${timestamp}.rtf`,
      filters: [{
        name: 'Rich Text Format',
        extensions: ['rtf']
      }, {
        name: 'Word Document',
        extensions: ['doc']
      }]
    });

    if (filePath) {
      await writeTextFile(filePath, rtfContent);
      console.log('Exported to:', filePath);
      // Visual feedback
      exportDocxBtn.textContent = '✅ DOCX';
      setTimeout(() => {
        exportDocxBtn.textContent = '📘 DOCX';
      }, 2000);
    }
  } catch (error) {
    console.error('Export failed:', error);
    alert('Export failed: ' + error);
  }
});

