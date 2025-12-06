// ============================================================================
// File Editor Mode Functions
// ============================================================================

// File Editor State
let fileEditorState = {
    filePath: null,
    fileName: null,
    fileContent: null,
    fileExtension: null,
    fileSize: 0
};

// Setup mode handler
function setupModeHandler() {
    const radios = document.querySelectorAll('input[name="mode"]');
    if (radios.length === 0) {
        console.warn('Mode radio buttons not found');
        return;
    }

    radios.forEach(radio => {
        radio.addEventListener('change', (e) => {
            const mode = e.target.value;
            const selectFileBtn = document.getElementById('select-file-btn');

            console.log('Mode changed to:', mode);

            if (mode === 'file-editor') {
                selectFileBtn.disabled = false;
                selectFileBtn.style.opacity = '1';
                console.log('File editor enabled');
            } else {
                // Switching away from file-editor - clear file preview
                selectFileBtn.disabled = true;
                selectFileBtn.style.opacity = '0.5';

                // Clear file editor state
                clearFileEditorPreview();
                console.log('File editor disabled, preview cleared');
            }
        });
    });

    console.log('Mode handler setup complete');
}

// Clear file editor preview and state
function clearFileEditorPreview() {
    // Reset file editor state
    fileEditorState = {
        filePath: null,
        fileName: null,
        fileContent: null,
        fileExtension: null,
        fileSize: 0
    };

    // Clear UI elements
    const selectedFileInfo = document.getElementById('selected-file-info');
    const filePreview = document.getElementById('file-preview');
    const fileNameElement = document.getElementById('selected-file-name');
    const fileSizeElement = document.getElementById('selected-file-size');
    const fileContentPreview = document.getElementById('file-content-preview');

    if (selectedFileInfo) {
        selectedFileInfo.classList.add('hidden');
    }

    if (filePreview) {
        filePreview.classList.add('hidden');
    }

    if (fileNameElement) {
        fileNameElement.textContent = '';
        fileNameElement.onclick = null;
        fileNameElement.style.cursor = '';
        fileNameElement.style.textDecoration = '';
    }

    if (fileSizeElement) {
        fileSizeElement.textContent = '';
    }

    if (fileContentPreview) {
        fileContentPreview.textContent = '';
    }

    // Update global state
    window.fileEditorState = fileEditorState;
}


// File selection handler
document.addEventListener('DOMContentLoaded', () => {
    console.log('File editor module loaded');

    // Setup mode handler
    setupModeHandler();

    const selectFileBtn = document.getElementById('select-file-btn');
    if (!selectFileBtn) {
        console.warn('Select file button not found');
        return;
    }

    console.log('File selection button found');

    selectFileBtn.addEventListener('click', async () => {
        try {
            const { open } = window.__TAURI__.dialog;
            const { readTextFile } = window.__TAURI__.fs;

            const filePath = await open({
                multiple: false,
                filters: [
                    {
                        name: 'Text Files',
                        extensions: ['txt', 'md', 'json', 'js', 'ts', 'jsx', 'tsx', 'py', 'rs', 'go', 'java', 'cpp', 'c', 'h', 'hpp', 'css', 'html', 'xml', 'yaml', 'yml', 'toml', 'ini', 'conf', 'sh', 'bash', 'zsh', 'fish', 'ps1', 'bat', 'cmd']
                    },
                    {
                        name: 'Code Files',
                        extensions: ['js', 'ts', 'jsx', 'tsx', 'py', 'rs', 'go', 'java', 'cpp', 'c', 'h', 'hpp', 'cs', 'php', 'rb', 'swift', 'kt', 'scala', 'clj', 'hs', 'ml', 'fs', 'vb', 'sql', 'r', 'm', 'mm']
                    },
                    {
                        name: 'Markup & Config',
                        extensions: ['html', 'xml', 'json', 'yaml', 'yml', 'toml', 'ini', 'conf', 'cfg', 'env', 'properties']
                    },
                    {
                        name: 'All Files',
                        extensions: ['*']
                    }
                ]
            });

            if (filePath) {
                // Normalize path for Windows (remove leading slash if present, normalize separators)
                let normalizedPath = filePath;
                // Remove leading slash on Windows paths like "/C:\Users\..."
                if (normalizedPath.startsWith('/') && normalizedPath.match(/^\/[A-Z]:/)) {
                    normalizedPath = normalizedPath.substring(1);
                }
                // Normalize path separators
                normalizedPath = normalizedPath.replace(/\\/g, '/');
                
                const content = await readTextFile(normalizedPath);
                const pathParts = normalizedPath.split('/');
                const fileName = pathParts[pathParts.length - 1];
                const nameParts = fileName.split('.');
                const extension = nameParts.length > 1 ? nameParts[nameParts.length - 1] : '';

                fileEditorState = {
                    filePath: normalizedPath,
                    fileName,
                    fileContent: content,
                    fileExtension: extension,
                    fileSize: content.length
                };

                // Update UI - make file name clickable
                const fileNameElement = document.getElementById('selected-file-name');
                fileNameElement.textContent = fileName;
                fileNameElement.style.cursor = 'pointer';
                fileNameElement.style.textDecoration = 'underline';
                fileNameElement.onclick = async () => {
                    try {
                        // First, try to open the parent folder
                        const pathParts = filePath.split('/');
                        pathParts.pop(); // Remove filename
                        const folderPath = pathParts.join('/');

                        if (window.__TAURI__?.opener && folderPath) {
                            try {
                                await window.__TAURI__.opener.open(folderPath);
                                console.log('Opened folder:', folderPath);
                                return; // Success
                            } catch (folderError) {
                                console.log('Folder open failed:', folderError);
                                // Fall through to show manual dialog
                            }
                        }

                        // If opener fails or unavailable, copy path and show message
                        throw new Error('Automatic opening not available');

                    } catch (e) {
                        console.error('Opening file/folder:', e);

                        // Get folder path
                        const pathParts = filePath.split('/');
                        pathParts.pop();
                        const folderPath = pathParts.join('/');

                        // Copy to clipboard first
                        try {
                            await navigator.clipboard.writeText(folderPath);
                            console.log('Copied folder path to clipboard');
                        } catch (clipErr) {
                            console.error('Failed to copy:', clipErr);
                        }

                        // Detect OS for instructions
                        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
                        const isWindows = navigator.platform.toUpperCase().indexOf('WIN') >= 0;

                        let instruction = '';
                        if (isMac) {
                            instruction = 'Нажмите Cmd+Shift+G в Finder и вставьте путь.';
                        } else if (isWindows) {
                            instruction = 'Откройте проводник (Win+E), вставьте путь в адресную строку.';
                        } else {
                            instruction = 'Откройте файловый менеджер и вставьте путь.';
                        }

                        // Show message
                        const { message } = window.__TAURI__.dialog;
                        await message(
                            `📂 Путь к папке скопирован в буфер обмена!\n\n` +
                            `Файл: ${fileEditorState.fileName}\n` +
                            `Папка: ${folderPath}\n\n` +
                            instruction,
                            {
                                title: 'Расположение файла',
                                type: 'info'
                            }
                        );
                    }
                };

                document.getElementById('selected-file-size').textContent = `${(content.length / 1024).toFixed(2)} KB`;
                document.getElementById('selected-file-info').classList.remove('hidden');

                // Show preview
                const preview = content.length > 500 ? content.substring(0, 500) + '\n\n... (truncated)' : content;
                document.getElementById('file-content-preview').textContent = preview;
                document.getElementById('file-preview').classList.remove('hidden');

            }
        } catch (error) {
            console.error('Failed to read file:', error);
            alert('Failed to read file: ' + error);
        }
    });
});

// Handle File Editor mode processing
async function handleFileEditorMode(instructions) {
    if (!fileEditorState.filePath) {
        alert('Please select a file first!');
        return;
    }

    // Get references to DOM elements
    const sendBtn = document.getElementById('send-btn');
    const abortBtn = document.getElementById('abort-btn');
    const providerSelect = document.getElementById('provider');
    const useEncryption = document.getElementById('use-encryption');
    const encryptionKeyInput = document.getElementById('encryption-key');
    const promptInput = document.getElementById('prompt-input'); // Added for finally block

    // Access global functions from main.js
    const appendMessage = window.appendMessage;
    const invoke = window.__TAURI__.core.invoke;

    // Set processing state
    window.isProcessing = true;
    window.abortRequested = false; // Reset abort flag

    if (sendBtn) {
        sendBtn.disabled = true;
        sendBtn.textContent = '⏳ Processing...';
    }

    // Show abort button
    if (abortBtn) {
        abortBtn.classList.remove('hidden');
        console.log('Abort button shown in file editor mode');
    }

    try {
        // Build prompt with file content
        const fullPrompt = `File: ${fileEditorState.fileName}
Extension: .${fileEditorState.fileExtension}

Current Content:
\`\`\`
${fileEditorState.fileContent}
\`\`\`

Instructions: ${instructions}

Please provide the modified file content. Return ONLY the file content, without any explanations or markdown code blocks.`;

        // Show user message
        if (appendMessage) {
            appendMessage(`📝 Editing file: ${fileEditorState.fileName}\n\nInstructions: ${instructions}`, 'user');
        }

        // Send to AI
        const provider = providerSelect ? providerSelect.value : 'telegram';
        const apiKey = document.getElementById('api-key')?.value || '';

        let telegramUrl = null;
        let encryptionKey = null;
        let useEnc = false;

        if (provider === 'telegram') {
            const host = document.getElementById('telegram-url')?.value || '';
            const port = document.getElementById('telegram-port')?.value || '8000';

            if (host.includes('://')) {
                telegramUrl = host;
            } else {
                telegramUrl = `http://${host}:${port}/ai_query`;
            }

            // Fix: useEncryption is a hidden input
            if (window.appConfig && window.appConfig.api_keys) {
                useEnc = window.appConfig.api_keys.telegram_use_encryption;
            } else {
                useEnc = useEncryption ? (useEncryption.value === 'true') : false;
            }

            if (useEnc && encryptionKeyInput) {
                encryptionKey = encryptionKeyInput.value;
            }
        }

        // Log network request to history sidebar
        if (window.logNetworkRequest) {
            window.logNetworkRequest(telegramUrl || 'Direct API', useEnc, `File Editor: ${fileEditorState.fileName}\nInstructions: ${instructions}`);
        }

        const response = await invoke('perform_search', {
            query: fullPrompt,
            provider,
            apiKey,
            telegramUrl,
            encryptionKey,
            useEncryption: useEnc,
            chatMode: false,
            conversationId: null
        });

        // Check if aborted
        if (window.abortRequested) {
            appendMessage('⚠️ File processing cancelled by user', 'system');
            return;
        }

        // Show AI response preview
        const previewText = response.text.length > 300
            ? response.text.substring(0, 300) + '...'
            : response.text;
        appendMessage(`✅ File processed!\n\nPreview:\n${previewText}`, 'ai');

        // Ask user: overwrite or create new?
        const { ask } = window.__TAURI__.dialog;

        // First dialog: Do you want to save?
        const shouldSave = await ask(
            'Do you want to save the AI-processed file?',
            {
                title: 'Save File',
                type: 'warning'
            }
        );

        if (!shouldSave) {
            // User cancelled (clicked No)
            appendMessage('❌ Save cancelled', 'system');
            return;
        }

        // Second dialog: Overwrite or create new?
        const shouldOverwrite = await ask(
            'YES - Overwrite original file\\n\\nNO - Create new file with _AI_N suffix',
            {
                title: 'Save Options',
                type: 'warning'
            }
        );

        const { writeTextFile } = window.__TAURI__.fs;
        let savePath;

        if (shouldOverwrite) {
            // Overwrite original - normalize path
            savePath = fileEditorState.filePath;
            // Normalize path for Windows
            if (savePath.startsWith('/') && savePath.match(/^\/[A-Z]:/)) {
                savePath = savePath.substring(1);
            }
            savePath = savePath.replace(/\\/g, '/');
            await writeTextFile(savePath, response.text);
            appendMessage(`💾 File overwritten: ${fileEditorState.fileName}`, 'system');
        } else {
            // Create new with version
            const versionedPath = await getVersionedFilePath(fileEditorState.filePath);
            savePath = versionedPath;
            await writeTextFile(savePath, response.text);

            const versionedName = versionedPath.split('/').pop();
            appendMessage(`💾 New file created: ${versionedName}`, 'system');
        }

        scrollToBottom();

    } catch (error) {
        console.error('File editor error:', error);

        // Check if aborted
        if (window.abortRequested) {
            appendMessage('⚠️ File processing cancelled by user', 'system');
        } else {
            appendMessage(`❌ Error processing file: ${error}`, 'error');
        }
    } finally {
        window.isProcessing = false;
        window.abortRequested = false;

        if (sendBtn) {
            sendBtn.disabled = false;
            sendBtn.textContent = '→';
        }

        // Hide abort button
        const abortBtn = document.getElementById('abort-btn');
        if (abortBtn) {
            abortBtn.classList.add('hidden');
            console.log('Abort button hidden in file editor mode');
        }

        if (promptInput) {
            promptInput.value = '';
        }
        if (window.scrollToBottom) {
            window.scrollToBottom();
        }
    }
}

// Get versioned file path (_AI_1, _AI_2, etc)
async function getVersionedFilePath(originalPath) {
    const { exists } = window.__TAURI__.fs;

    const pathParts = originalPath.split('/');
    const fileName = pathParts.pop();
    const dir = pathParts.join('/');

    const nameParts = fileName.split('.');
    const extension = nameParts.length > 1 ? nameParts.pop() : '';
    const baseName = nameParts.join('.');

    let version = 1;
    let newPath;

    do {
        const newFileName = extension
            ? `${baseName}_AI_${version}.${extension}`
            : `${baseName}_AI_${version}`;
        newPath = `${dir}/${newFileName}`;
        version++;
    } while (await exists(newPath));

    return newPath;
}

// Export for use in main sendMessage
window.handleFileEditorMode = handleFileEditorMode;
window.fileEditorState = fileEditorState;
