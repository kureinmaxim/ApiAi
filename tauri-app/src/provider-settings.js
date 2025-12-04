// Provider Settings Modal Handler
document.addEventListener('DOMContentLoaded', () => {
    const openProviderSettings = document.getElementById('open-provider-settings');
    const providerSettingsModal = document.getElementById('provider-settings-modal');
    const saveProviderSettings = document.getElementById('save-provider-settings');
    const cancelProviderSettings = document.getElementById('cancel-provider-settings');
    const modalShowKeys = document.getElementById('modal-show-keys');

    if (!openProviderSettings || !providerSettingsModal) return;

    // Open modal
    openProviderSettings.addEventListener('click', () => {
        // Check if settings are unlocked
        if (!window.settingsUnlocked) {
            // Trigger PIN modal if locked
            if (window.showPinModal) {
                // Set callback to open settings after unlock
                window.pinSuccessCallback = () => {
                    openProviderSettings.click();
                };
                window.showPinModal();
            } else {
                alert('Please unlock settings first (click the lock icon)');
            }
            return;
        }

        // Load current values
        document.getElementById('modal-telegram-url').value = document.getElementById('telegram-url')?.value || '';
        document.getElementById('modal-telegram-port').value = document.getElementById('telegram-port')?.value || '';
        document.getElementById('modal-api-key').value = document.getElementById('modal-api-key')?.value || '';
        document.getElementById('modal-encryption-key').value = document.getElementById('encryption-key')?.value || '';
        document.getElementById('modal-use-encryption').checked = document.getElementById('use-encryption')?.value === 'true';
        document.getElementById('modal-show-keys').checked = false;

        providerSettingsModal.classList.remove('hidden');
    });

    // Close modal
    const closeModal = () => {
        providerSettingsModal.classList.add('hidden');
    };

    cancelProviderSettings?.addEventListener('click', closeModal);

    // Save settings
    saveProviderSettings?.addEventListener('click', async () => {
        // Save to hidden fields first
        const telegramUrl = document.getElementById('telegram-url');
        const telegramPort = document.getElementById('telegram-port');
        const apiKey = document.getElementById('api-key');
        const encryptionKey = document.getElementById('encryption-key');
        const useEncryption = document.getElementById('use-encryption');

        const modalTelegramUrl = document.getElementById('modal-telegram-url').value;
        const modalTelegramPort = document.getElementById('modal-telegram-port').value;
        const modalApiKey = document.getElementById('modal-api-key').value;
        const modalEncryptionKey = document.getElementById('modal-encryption-key').value;
        const modalUseEncryption = document.getElementById('modal-use-encryption').checked;

        if (telegramUrl) telegramUrl.value = modalTelegramUrl;
        if (telegramPort) telegramPort.value = modalTelegramPort;
        if (apiKey) apiKey.value = modalApiKey;
        if (encryptionKey) encryptionKey.value = modalEncryptionKey;
        if (useEncryption) useEncryption.value = modalUseEncryption;

        // Save to backend config file
        if (window.appConfig) {
            const newConfig = { ...window.appConfig };

            // Update config with modal values
            newConfig.api_keys.telegram_url = modalTelegramUrl;
            newConfig.api_keys.telegram_key = modalApiKey;
            newConfig.api_keys.telegram_enc_key = modalEncryptionKey;
            newConfig.api_keys.telegram_use_encryption = modalUseEncryption;

            try {
                const { invoke } = window.__TAURI__.core;
                await invoke('save_config', { newConfig });

                // Update global config
                window.appConfig = newConfig;

                console.log('Provider settings saved to config file');
            } catch (e) {
                console.error('Failed to save config:', e);
                alert('Ошибка сохранения: ' + e);
                return;
            }
        }

        closeModal();

        // Visual feedback
        saveProviderSettings.textContent = '✅ Сохранено!';
        setTimeout(() => {
            saveProviderSettings.textContent = '💾 Save Settings';
        }, 2000);
    });

    // Show/hide keys toggle
    modalShowKeys?.addEventListener('change', (e) => {
        const apiKeyInput = document.getElementById('modal-api-key');
        const encKeyInput = document.getElementById('modal-encryption-key');

        if (e.target.checked) {
            apiKeyInput.type = 'text';
            encKeyInput.type = 'text';
        } else {
            apiKeyInput.type = 'password';
            encKeyInput.type = 'password';
        }
    });
});
