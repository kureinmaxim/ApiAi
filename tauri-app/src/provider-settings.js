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
    saveProviderSettings?.addEventListener('click', () => {
        // Save to hidden fields
        const telegramUrl = document.getElementById('telegram-url');
        const telegramPort = document.getElementById('telegram-port');
        const apiKey = document.getElementById('api-key');
        const encryptionKey = document.getElementById('encryption-key');
        const useEncryption = document.getElementById('use-encryption');

        if (telegramUrl) telegramUrl.value = document.getElementById('modal-telegram-url').value;
        if (telegramPort) telegramPort.value = document.getElementById('modal-telegram-port').value;
        if (apiKey) apiKey.value = document.getElementById('modal-api-key').value;
        if (encryptionKey) encryptionKey.value = document.getElementById('modal-encryption-key').value;
        if (useEncryption) useEncryption.value = document.getElementById('modal-use-encryption').checked;

        closeModal();

        // Visual feedback
        saveProviderSettings.textContent = '✅ Saved!';
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
