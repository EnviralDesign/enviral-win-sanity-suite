//! Application styles
//!
//! Beautiful dark theme CSS with modern aesthetics.

/// Main CSS styles for the application
pub const STYLES: &str = r#"
/* ========================================
   Sanity Suite - Modern Dark Theme
   ======================================== */

/* CSS Variables */
:root {
    /* Colors - Deep dark with purple/blue accents */
    --bg-primary: #0f0f14;
    --bg-secondary: #16161e;
    --bg-tertiary: #1e1e28;
    --bg-elevated: #24242e;
    --bg-hover: #2a2a36;
    
    --text-primary: #e4e4e8;
    --text-secondary: #9898a6;
    --text-muted: #6b6b7a;
    
    --accent-primary: #6366f1;
    --accent-secondary: #818cf8;
    --accent-muted: rgba(99, 102, 241, 0.15);
    
    --success: #22c55e;
    --success-muted: rgba(34, 197, 94, 0.15);
    --warning: #f59e0b;
    --warning-muted: rgba(245, 158, 11, 0.15);
    --error: #ef4444;
    --error-muted: rgba(239, 68, 68, 0.15);
    
    --border-color: rgba(255, 255, 255, 0.08);
    --border-active: rgba(99, 102, 241, 0.5);
    
    /* Spacing */
    --space-xs: 4px;
    --space-sm: 8px;
    --space-md: 16px;
    --space-lg: 24px;
    --space-xl: 32px;
    
    /* Border Radius */
    --radius-sm: 6px;
    --radius-md: 10px;
    --radius-lg: 14px;
    --radius-xl: 20px;
    
    /* Typography */
    --font-sans: 'Segoe UI', system-ui, -apple-system, sans-serif;
    --font-mono: 'Cascadia Code', 'Consolas', monospace;
    
    /* Shadows */
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
    --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.4);
    --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.5);
    --shadow-glow: 0 0 20px rgba(99, 102, 241, 0.3);
    
    /* Transitions */
    --transition-fast: 150ms cubic-bezier(0.4, 0, 0.2, 1);
    --transition-normal: 250ms cubic-bezier(0.4, 0, 0.2, 1);
}

/* Base Styles */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

html, body {
    width: 100%;
    height: 100%;
    overflow: hidden;
}

body {
    font-family: var(--font-sans);
    font-size: 14px;
    line-height: 1.5;
    color: var(--text-primary);
    background: var(--bg-primary);
    -webkit-font-smoothing: antialiased;
}

/* ========================================
   Layout
   ======================================== */

.app-container {
    display: flex;
    width: 100%;
    height: 100vh;
    background: var(--bg-primary);
}

/* Sidebar */
.sidebar {
    width: 220px;
    min-width: 220px;
    height: 100%;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: var(--space-md);
}

.sidebar-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) 0;
    margin-bottom: var(--space-lg);
}

.sidebar-logo {
    font-size: 20px;
    font-weight: 700;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    flex: 1;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
    border: 1px solid transparent;
    background: transparent;
    font-size: 14px;
    font-family: var(--font-sans);
    text-align: left;
    width: 100%;
}

.nav-item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
}

.nav-item.active {
    background: var(--accent-muted);
    color: var(--accent-secondary);
    border-color: var(--accent-primary);
}

.nav-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
}

.sidebar-footer {
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-color);
    margin-top: auto;
}

.admin-badge {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    font-size: 12px;
}

.admin-badge.elevated {
    background: var(--success-muted);
    color: var(--success);
}

.admin-badge.standard {
    background: var(--bg-tertiary);
    color: var(--text-muted);
}

/* Main Content */
.main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary);
}

.page-header {
    padding: var(--space-lg);
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-secondary);
}

.page-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
}

.page-subtitle {
    font-size: 14px;
    color: var(--text-muted);
}

.page-content {
    flex: 1;
    padding: var(--space-lg);
    overflow-y: auto;
}

/* ========================================
   Components
   ======================================== */

/* Buttons */
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    font-size: 14px;
    font-weight: 500;
    font-family: var(--font-sans);
    cursor: pointer;
    transition: all var(--transition-fast);
    border: 1px solid transparent;
    white-space: nowrap;
}

.btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.btn-primary {
    background: var(--accent-primary);
    color: white;
    border-color: var(--accent-primary);
}

.btn-primary:hover:not(:disabled) {
    background: var(--accent-secondary);
    box-shadow: var(--shadow-glow);
}

.btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    border-color: var(--border-color);
}

.btn-secondary:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--border-active);
}

.btn-danger {
    background: var(--error);
    color: white;
    border-color: var(--error);
}

.btn-danger:hover:not(:disabled) {
    background: #dc2626;
}

.btn-ghost {
    background: transparent;
    color: var(--text-secondary);
    border-color: transparent;
}

.btn-ghost:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
}

.btn-warning {
    background: var(--warning);
    color: white;
    border-color: var(--warning);
}

.btn-warning:hover:not(:disabled) {
    background: #d97706;
}

.btn-sm {
    padding: var(--space-xs) var(--space-sm);
    font-size: 12px;
}

/* Input */
.input-group {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
}

.input {
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: 14px;
    font-family: var(--font-sans);
    transition: all var(--transition-fast);
    outline: none;
}

.input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px var(--accent-muted);
}

.input::placeholder {
    color: var(--text-muted);
}

.input-number {
    width: 120px;
    text-align: center;
}

/* Tables */
.data-table {
    width: 100%;
    border-collapse: collapse;
    background: var(--bg-secondary);
    border-radius: var(--radius-lg);
    overflow: hidden;
    border: 1px solid var(--border-color);
}

.data-table th {
    padding: var(--space-md);
    text-align: left;
    font-weight: 600;
    color: var(--text-secondary);
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border-color);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.data-table td {
    padding: var(--space-md);
    border-bottom: 1px solid var(--border-color);
    color: var(--text-primary);
}

.data-table tr:last-child td {
    border-bottom: none;
}

.data-table tr:hover td {
    background: var(--bg-hover);
}

.data-table .mono {
    font-family: var(--font-mono);
    font-size: 13px;
}

.data-table .conflict {
    color: var(--error);
}

.data-table .muted {
    color: var(--text-muted);
}

.data-table .orphan {
    color: var(--warning);
}

.data-table .success {
    color: var(--success);
}

.data-table tr.selected {
    background: var(--accent-muted);
    border-color: var(--accent-primary);
}

.data-table tr.selected td {
    color: var(--text-primary);
}

/* Service List */
.service-list {
    max-height: 300px;
    overflow-y: auto;
    border-radius: var(--radius-lg);
}

/* Action Bar */
.action-bar {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-md);
    background: var(--bg-secondary);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    margin-bottom: var(--space-lg);
}

.action-bar-group {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
}

.action-bar-divider {
    width: 1px;
    height: 24px;
    background: var(--border-color);
}

/* Status Bar */
.status-bar {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    margin-top: var(--space-md);
    font-size: 13px;
    color: var(--text-secondary);
}

.status-bar.success {
    background: var(--success-muted);
    border-color: var(--success);
    color: var(--success);
}

.status-bar.error {
    background: var(--error-muted);
    border-color: var(--error);
    color: var(--error);
}

.status-bar.warning {
    background: var(--warning-muted);
    border-color: var(--warning);
    color: var(--warning);
}

/* Output Panel - Fixed at bottom */
.output-panel {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.6;
    overflow: auto;
    max-height: 300px;
}

.output-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    font-family: var(--font-sans);
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.output-panel-content {
    padding: var(--space-md);
    white-space: pre-wrap;
    color: var(--text-primary);
}

/* Split Layout - scrollable top, fixed output bottom */
.page-split-layout {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
}

.page-controls {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
    padding-bottom: 0;
}

.output-panel-container {
    flex-shrink: 0;
    padding: var(--space-md) var(--space-lg) var(--space-lg);
    background: var(--bg-primary);
}

.output-panel-fixed {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    display: flex;
    flex-direction: column;
    height: 200px;
}

.output-panel-fixed .output-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
    flex-shrink: 0;
}

.output-panel-title {
    font-family: var(--font-sans);
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.output-panel-actions {
    display: flex;
    gap: var(--space-sm);
}

.output-panel-fixed .output-panel-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-md);
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
}

.output-panel-empty {
    color: var(--text-muted);
    font-family: var(--font-sans);
    font-style: italic;
}

.output-text {
    margin: 0;
    font-family: var(--font-mono);
    white-space: pre-wrap;
    word-break: break-word;
}

/* Card */
.card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
}

.card-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-md);
}

/* Empty State */
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-xl);
    color: var(--text-muted);
    text-align: center;
}

.empty-state-icon {
    font-size: 48px;
    margin-bottom: var(--space-md);
    opacity: 0.5;
}

.empty-state-text {
    font-size: 14px;
}

/* Loading Spinner */
.spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}

@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

/* Quick Actions Grid */
.quick-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
}

.quick-action-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-md);
    min-width: 100px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast);
    font-family: var(--font-sans);
}

.quick-action-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
}

.quick-action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.quick-action-btn.quick-action-warning {
    border-color: var(--warning);
    background: var(--warning-muted);
}

.quick-action-btn.quick-action-warning:hover:not(:disabled) {
    border-color: var(--warning);
    background: rgba(245, 158, 11, 0.25);
}

.quick-action-icon {
    font-size: 24px;
}

.quick-action-label {
    font-size: 12px;
    font-weight: 500;
}

/* Section */
.section {
    margin-bottom: var(--space-xl);
}

.section-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-md);
}

/* Stats Grid */
.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--space-md);
}

.stat-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
}

.stat-card-warning {
    border-color: var(--warning);
    background: var(--warning-muted);
}

.stat-card-danger {
    border-color: var(--error);
    background: var(--error-muted);
}

.stat-value {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
}

.stat-label {
    font-size: 12px;
    color: var(--text-muted);
}

/* Progress Bar */
.progress-bar {
    height: 6px;
    background: var(--bg-tertiary);
    border-radius: 3px;
    overflow: hidden;
    margin-top: var(--space-xs);
}

.progress-fill {
    height: 100%;
    background: var(--accent-primary);
    border-radius: 3px;
    transition: width 0.3s ease;
}

.stat-card-warning .progress-fill {
    background: var(--warning);
}

.stat-card-danger .progress-fill {
    background: var(--error);
}
"#;
