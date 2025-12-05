// ============================================================================
// Network Monitor - Request History and Console Access
// ============================================================================

// Request history storage
let requestHistory = [];
const MAX_HISTORY_SIZE = 100;

// Track requests globally
window.trackNetworkRequest = function (requestData) {
    const historyItem = {
        timestamp: new Date().toLocaleTimeString(),
        method: 'POST',
        endpoint: requestData.url || '/ai_query',
        encrypted: requestData.encrypted || false,
        status: requestData.status || 'pending',
        payload: requestData.payload,
        response: requestData.response
    };

    requestHistory.unshift(historyItem);

    // Keep only last MAX_HISTORY_SIZE items
    if (requestHistory.length > MAX_HISTORY_SIZE) {
        requestHistory = requestHistory.slice(0, MAX_HISTORY_SIZE);
    }

    updateHistoryUI();
};

// Update history UI
function updateHistoryUI() {
    const historyList = document.getElementById('request-history-list');
    const filter = document.getElementById('history-filter')?.value || 'all';

    // Filter requests
    let filtered = requestHistory;
    if (filter === 'encrypted') {
        filtered = requestHistory.filter(r => r.encrypted);
    } else if (filter === 'unencrypted') {
        filtered = requestHistory.filter(r => !r.encrypted);
    }

    if (filtered.length === 0) {
        historyList.innerHTML = `
            <div class="request-history-empty">
                <div class="history-empty-icon">📡</div>
                <div class="history-empty-text">No requests yet</div>
            </div>
        `;
        return;
    }

    historyList.innerHTML = filtered.map(item => {
        const encryptionClass = item.encrypted ? 'encrypted' : 'unencrypted';
        const encryptionIcon = item.encrypted ? '🔒' : '⚠️';
        const endpoint = item.endpoint.split('/').pop() || item.endpoint;

        return `
            <div class="request-item ${encryptionClass}">
                <div class="request-time">${item.timestamp}</div>
                <div class="request-info">
                    <span class="request-status">${encryptionIcon}</span>
                    <span class="request-method">POST</span>
                    <span class="request-endpoint">${endpoint}</span>
                </div>
            </div>
        `;
    }).join('');
}

// Console modal controls
document.addEventListener('DOMContentLoaded', () => {
    const consoleBtn = document.getElementById('console-btn');
    const consoleModal = document.getElementById('console-modal');
    const consoleClose = document.getElementById('console-close');
    const historyFilter = document.getElementById('history-filter');
    const toggleHistory = document.getElementById('toggle-history');
    const historySidebar = document.getElementById('request-history-sidebar');
    const container = document.querySelector('.container');

    // Console modal
    if (consoleBtn && consoleModal && consoleClose) {
        consoleBtn.addEventListener('click', () => {
            consoleModal.classList.remove('hidden');
        });

        consoleClose.addEventListener('click', () => {
            consoleModal.classList.add('hidden');
        });
    }

    // History filter
    if (historyFilter) {
        historyFilter.addEventListener('change', () => {
            updateHistoryUI();
        });
    }

    // Toggle history sidebar
    if (toggleHistory && historySidebar && container) {
        let isCollapsed = true; // Start collapsed by default

        toggleHistory.addEventListener('click', () => {
            isCollapsed = !isCollapsed;

            if (isCollapsed) {
                historySidebar.classList.add('collapsed');
                container.classList.remove('history-visible');
                toggleHistory.textContent = '📡'; // Show icon when collapsed
                toggleHistory.classList.add('collapsed');
                toggleHistory.title = "Show Network Log";
            } else {
                historySidebar.classList.remove('collapsed');
                container.classList.add('history-visible');
                toggleHistory.textContent = '»'; // Arrow to collapse right
                toggleHistory.classList.remove('collapsed');
                toggleHistory.title = "Hide Network Log";
            }
        });

        // Start with sidebar collapsed
        historySidebar.classList.add('collapsed');
        container.classList.remove('history-visible');
        toggleHistory.textContent = '📡';
        toggleHistory.classList.add('collapsed');
        toggleHistory.title = "Show Network Log";
    }

    console.log('Network monitor initialized');
});

// Hook into existing sendMessage to track requests
// This should be called from main.js after making a request
window.logNetworkRequest = function (url, encrypted, payload) {
    window.trackNetworkRequest({
        url,
        encrypted,
        payload,
        status: 'sent'
    });
};

// Clear network log
window.clearNetworkLog = function () {
    requestHistory = [];
    updateHistoryUI();
};

// Export for use in main.js
window.updateHistoryUI = updateHistoryUI;

