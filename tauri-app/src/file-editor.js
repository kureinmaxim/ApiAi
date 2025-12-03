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
                selectFileBtn.disabled = true;
                selectFileBtn.style.opacity = '0.5';
            }
        });
    });

    console.log('Mode handler setup complete');
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
                const content = await readTextFile(filePath);
                const pathParts = filePath.split('/');
                const fileName = pathParts[pathParts.length - 1];
                const nameParts = fileName.split('.');
                const extension = nameParts.length > 1 ? nameParts[nameParts.length - 1] : '';

                fileEditorState = {
                    filePath,
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
                        // Use opener plugin (Tauri v2) - open file with default system app
                        // Try to open the file directly first
                        if (window.__TAURI__?.opener) {
                            try {
                                await window.__TAURI__.opener.open(filePath);
                                return; // Success
                            } catch (fileError) {
                                console.log('Direct file open failed, trying folder:', fileError);
                                // If direct file open fails, try opening the folder containing the file
                                const pathParts = filePath.split('/');
                                pathParts.pop(); // Remove filename
                                const folderPath = pathParts.join('/');
                                if (folderPath) {
                                    await window.__TAURI__.opener.open(folderPath);
                                    return; // Success
                                }
                                throw fileError; // Re-throw if folder open also fails
                            }
                        } else {
                            throw new Error('Opener plugin not available');
                        }
                    } catch (e) {
                        console.error('Failed to open file:', e);
                        // Fallback: show file path so user can open manually
                        const { message } = window.__TAURI__.dialog;
                        await message(`Could not open file automatically.\n\nFile path:\n${filePath}\n\nYou can copy this path and open it manually.`, {
                            title: 'File Location',
                            type: 'info',
                            okLabel: 'OK'
                        });
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

    isProcessing = true;
    sendBtn.disabled = true;
    sendBtn.textContent = '⏳ Processing...';

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
        appendMessage(`📝 Editing file: ${fileEditorState.fileName}\n\nInstructions: ${instructions}`, 'user');

        // Send to AI
        const provider = providerSelect.value;
        const apiKey = document.getElementById('api-key').value;

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

        // Show AI response preview
        const previewText = response.text.length > 300
            ? response.text.substring(0, 300) + '...'
            : response.text;
        appendMessage(`✅ File processed!\n\nPreview:\n${previewText}`, 'ai');

        // Ask user: overwrite or create new?
        const { ask } = window.__TAURI__.dialog;
        const choice = await ask(
            'Save Options:\n\n' +
            '• YES - Overwrite original file\n' +
            '• NO - Create new file with _AI_N suffix\n' +
            '• CANCEL - Don\'t save',
            {
                title: 'Save File',
                type: 'warning'
            }
        );

        if (choice === null) {
            // User cancelled
            appendMessage('❌ Save cancelled', 'system');
            return;
        }

        const { writeTextFile } = window.__TAURI__.fs;
        let savePath;

        if (choice) {
            // Overwrite original
            savePath = fileEditorState.filePath;
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
        appendMessage(`❌ Error: ${error}`, 'error');
    } finally {
        isProcessing = false;
        sendBtn.disabled = false;
        sendBtn.textContent = 'Send 🚀';
        promptInput.value = '';
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
