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

// DOM Elements - Window Size Modal
const windowSizeModal = document.getElementById('window-size-modal');
const windowSizeInfo = document.getElementById('window-size-info');
const windowSizeSave = document.getElementById('window-size-save');
const windowSizeReset = document.getElementById('window-size-reset');
const windowSizeCancel = document.getElementById('window-size-cancel');

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
  updateProviderInfo();
  setupWindowSizeHandler();

  try {
    // Load config from backend
    const config = await invoke('get_config');
    console.log('Loaded config:', config);

    // Restore saved window size if available
    if (config.ui && config.ui.window_width && config.ui.window_height) {
      try {
        const currentWindow = getCurrentWindow();
        const width = config.ui.window_width;
        const height = config.ui.window_height;
        console.log('Restoring window size (logical):', width, height);
        // setSize with logical size (will be converted to physical by Tauri)
        await currentWindow.setSize({ width, height });
        // Wait a bit for window to resize, then update display
        setTimeout(() => {
          updateWindowSize();
        }, 100);
      } catch (error) {
        console.error('Failed to restore window size:', error);
      }
    }

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
  
  // Force focus with requestAnimationFrame for better reliability
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      if (pinInput) {
        pinInput.focus();
        pinInput.select();
        // Also try click to ensure focus
        pinInput.click();
      }
    });
  });
  
  // Additional fallback attempts
  setTimeout(() => {
    if (pinInput && document.activeElement !== pinInput) {
      pinInput.focus();
      pinInput.select();
    }
  }, 50);
  
  setTimeout(() => {
    if (pinInput && document.activeElement !== pinInput) {
      pinInput.focus();
      pinInput.select();
    }
  }, 200);
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
    const currentWindow = getCurrentWindow();
    const physicalSize = await currentWindow.innerSize();
    const scaleFactor = await currentWindow.scaleFactor();
    
    // Convert to logical size for display
    const logicalWidth = Math.round(physicalSize.width / scaleFactor);
    const logicalHeight = Math.round(physicalSize.height / scaleFactor);
    
    windowSizeDisplay.textContent = `${logicalWidth}×${logicalHeight}`;
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

// Setup double-click handler for window size
function setupWindowSizeHandler() {
  // Wait for DOM to be ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setupWindowSizeHandler);
    return;
  }
  
  const windowSizeElement = document.getElementById('window-size');
  if (!windowSizeElement) {
    console.warn('Window size element not found, retrying...');
    setTimeout(setupWindowSizeHandler, 100);
    return;
  }

  console.log('Setting up window size handler on element:', windowSizeElement);
  
  let windowSizeClickCount = 0;
  let windowSizeClickTimer = null;

  // Handle double-click
  const handleDoubleClick = async (e) => {
    e.preventDefault();
    e.stopPropagation();
    
    console.log('Double-click detected on window size');
    
    try {
      const currentWindow = getCurrentWindow();
      const physicalSize = await currentWindow.innerSize();
      const scaleFactor = await currentWindow.scaleFactor();
      
      // Convert physical size to logical size (divide by scale factor)
      const logicalWidth = physicalSize.width / scaleFactor;
      const logicalHeight = physicalSize.height / scaleFactor;
      
      // Show modal with current size (display physical size, but save logical)
      windowSizeInfo.textContent = `Current size: ${Math.round(logicalWidth)}×${Math.round(logicalHeight)}`;
      windowSizeModal.classList.remove('hidden');
      
      // Store logical size for use in button handlers
      windowSizeModal.dataset.width = logicalWidth;
      windowSizeModal.dataset.height = logicalHeight;
    } catch (error) {
      console.error('Failed to get window size:', error);
      alert('Error: ' + error);
    }
  };
  
  // Window size modal handlers
  windowSizeSave.addEventListener('click', async () => {
    const width = parseFloat(windowSizeModal.dataset.width);
    const height = parseFloat(windowSizeModal.dataset.height);
    
    try {
      await invoke('save_window_size', {
        width: width,
        height: height
      });
      
      windowSizeModal.classList.add('hidden');
      
      // Visual feedback
      const originalText = windowSizeElement.textContent;
      windowSizeElement.textContent = '✅ Saved!';
      windowSizeElement.style.color = '#4ade80';
      
      setTimeout(() => {
        windowSizeElement.textContent = originalText;
        windowSizeElement.style.color = '';
      }, 2000);
    } catch (error) {
      console.error('Failed to save window size:', error);
      alert('Error: ' + error);
    }
  });
  
  windowSizeReset.addEventListener('click', async () => {
    try {
      await invoke('reset_window_size');
      
      const currentWindow = getCurrentWindow();
        await currentWindow.setSize({ width: 1200, height: 800 });
      
      windowSizeModal.classList.add('hidden');
      
      // Visual feedback
      const originalText = windowSizeElement.textContent;
      windowSizeElement.textContent = '↺ Reset!';
      windowSizeElement.style.color = '#fbbf24';
      
      setTimeout(() => {
        windowSizeElement.textContent = originalText;
        windowSizeElement.style.color = '';
        updateWindowSize();
      }, 2000);
    } catch (error) {
      console.error('Failed to reset window size:', error);
      alert('Error: ' + error);
    }
  });
  
  windowSizeCancel.addEventListener('click', () => {
    windowSizeModal.classList.add('hidden');
  });

  // Use dblclick event directly
  windowSizeElement.addEventListener('dblclick', handleDoubleClick);
  
  // Also add click handler as fallback (for better compatibility)
  windowSizeElement.addEventListener('click', (e) => {
    windowSizeClickCount++;
    
    if (windowSizeClickCount === 1) {
      windowSizeClickTimer = setTimeout(() => {
        windowSizeClickCount = 0;
      }, 300);
    } else if (windowSizeClickCount === 2) {
      clearTimeout(windowSizeClickTimer);
      windowSizeClickCount = 0;
      // Trigger double-click handler
      handleDoubleClick(e);
    }
  });
  
  console.log('Window size handler setup complete');
}

// Store current provider info from API responses
let currentProviderInfo = {
  provider: null,
  model: null
};

// Update provider info in header
function updateProviderInfo() {
  const providerInfo = document.getElementById('provider-info');
  if (!providerInfo) {
    // Element not found, try again later
    setTimeout(updateProviderInfo, 100);
    return;
  }
  
  const provider = providerSelect.value;
  
  let infoText = '';
  
  // For Telegram, use info from API response if available
  if (provider === 'telegram' && currentProviderInfo.provider && currentProviderInfo.model) {
    infoText = `${currentProviderInfo.provider} (${currentProviderInfo.model})`;
  } else if (provider === 'telegram') {
    // Show placeholder until first response
    infoText = 'AI via Telegram Server';
  } else {
    // Direct providers
    switch(provider) {
      case 'anthropic':
        infoText = 'Anthropic Claude (claude-3-sonnet-20240229)';
        break;
      case 'openai':
        infoText = 'OpenAI GPT (gpt-4o)';
        break;
      default:
        infoText = '';
    }
  }
  
  providerInfo.textContent = infoText;
}

// Event Listeners
providerSelect.addEventListener('change', () => {
  if (providerSelect.value === 'telegram') {
    telegramSettings.style.display = 'flex';
  } else {
    telegramSettings.style.display = 'none';
  }
  updateApiKeyField();
  updateProviderInfo();
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

// Echo button - sends echo test request automatically
const echoBtn = document.getElementById('echo-btn');
echoBtn.addEventListener('click', async () => {
  const currentText = promptInput.value.trim();
  const testMessage = currentText || 'Echo 123456789';

  // Prevented during processing
  if (isProcessing) return;

  // UI Updates
  appendMessage(`🔊 Echo Test: ${testMessage}`, 'user');
  isProcessing = true;
  echoBtn.disabled = true;
  echoBtn.textContent = '⏳ Testing...';

  // Measure request start time
  const startTime = performance.now();

  // Gather Settings
  const provider = providerSelect.value;

  if (provider !== 'telegram') {
    appendMessage('❌ Echo test only works with Telegram provider', 'error');
    isProcessing = false;
    echoBtn.disabled = false;
    echoBtn.textContent = '🔊 Echo';
    return;
  }

  const apiKey = document.getElementById('api-key').value;
  let telegramUrl = null;
  let encryptionKey = null;
  let useEnc = false;

  // Get Telegram URL
  const urlInput = document.getElementById('telegram-url').value;
  const portInput = document.getElementById('telegram-port').value;

  // Construct URL - replace /ai_query with /echo
  if (urlInput.includes('://')) {
    // Full URL provided, replace endpoint
    telegramUrl = urlInput.replace(/\/ai_query.*$/, '/echo');
  } else {
    // Host + port provided
    telegramUrl = `http://${urlInput}:${portInput}/echo`;
  }

  if (useEncryption.checked) {
    useEnc = true;
    encryptionKey = encryptionKeyInput.value;
  }

  try {
    const response = await invoke('perform_search', {
      query: testMessage,
      provider,
      apiKey,
      telegramUrl,
      encryptionKey,
      useEncryption: useEnc,
      chatMode: false,
      conversationId: null
    });

    // Measure round-trip time
    const endTime = performance.now();
    const roundTripTime = Math.round(endTime - startTime);

    // Server processing time from response
    const serverTime = response.processing_time_ms || 0;
    const networkTime = roundTripTime - serverTime;

    appendMessage(
      `✅ Echo Response: ${response.text}\n\n` +
      `📊 Timing:\n` +
      `• Round-trip: ${roundTripTime}ms\n` +
      `• Server: ${serverTime}ms\n` +
      `• Network: ${networkTime}ms`,
      'ai'
    );

  } catch (error) {
    const endTime = performance.now();
    const roundTripTime = Math.round(endTime - startTime);
    appendMessage(`❌ Echo Error: ${error}\n⏱️ Time: ${roundTripTime}ms`, 'error');
  } finally {
    isProcessing = false;
    echoBtn.disabled = false;
    echoBtn.textContent = '🔊 Echo';
    scrollToBottom();
  }
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

    // Remove provider information from response text if present
    let cleanedText = response.text
      .split('\n')
      .filter(line => {
        const trimmed = line.trim();
        return !trimmed.startsWith('Provider:') && 
               !trimmed.startsWith('provider:') &&
               !trimmed.includes('Provider:') &&
               !trimmed.includes('(Secure)');
      })
      .join('\n');
    
    appendMessage(cleanedText, 'ai', {
      provider: response.provider,
      model: response.model
    });

    // Update provider info for Telegram
    if (provider === 'telegram' && response.provider && response.model) {
      currentProviderInfo.provider = response.provider;
      currentProviderInfo.model = response.model;
      updateProviderInfo();
    }

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

function appendMessage(text, type, metadata = {}) {
  const div = document.createElement('div');
  div.className = `message ${type}`;

  // Simple markdown-like parsing for bold text
  const formattedText = text.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\n/g, '<br>');

  const contentDiv = document.createElement('div');
  contentDiv.className = 'content';
  contentDiv.innerHTML = formattedText;

  div.appendChild(contentDiv);

  // Don't add provider/model info to messages - it's shown in header instead

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

