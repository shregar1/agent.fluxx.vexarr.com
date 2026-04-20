/**
 * Fluxx RUM (Real User Monitoring) SDK
 *
 * Lightweight browser SDK for capturing:
 * - Page loads & navigation timing (Web Vitals: LCP, FID, CLS, TTFB, FCP)
 * - User sessions with page flow tracking
 * - JavaScript errors with stack traces
 * - XHR/Fetch network request timing
 * - Custom user actions and events
 * - Resource loading performance
 *
 * Usage:
 *   <script src="fluxx-rum.js"></script>
 *   <script>
 *     FluxxRUM.init({
 *       endpoint: 'http://localhost:7007',
 *       applicationId: 'my-web-app',
 *       sessionSampleRate: 100,
 *     });
 *   </script>
 *
 * Size: ~8KB minified (zero dependencies)
 */

(function (global) {
  'use strict';

  const VERSION = '1.0.0';

  // ── Config ──
  let config = {
    endpoint: '',
    applicationId: '',
    environment: 'production',
    version: '',
    sessionSampleRate: 100,
    trackErrors: true,
    trackResources: true,
    trackUserActions: true,
    trackNetworkRequests: true,
    maxEventsPerSession: 10000,
    flushIntervalMs: 5000,
    userId: null,
    userEmail: null,
    customAttributes: {},
  };

  // ── State ──
  let sessionId = generateId();
  let sessionStart = Date.now();
  let eventBuffer = [];
  let eventCount = 0;
  let flushTimer = null;
  let initialized = false;
  let pageViewId = generateId();

  // ── Public API ──

  const FluxxRUM = {
    VERSION,

    /**
     * Initialize the RUM SDK.
     */
    init(options) {
      if (initialized) return;
      Object.assign(config, options);

      if (!config.endpoint) {
        console.warn('[FluxxRUM] No endpoint configured');
        return;
      }

      // Session sampling
      if (Math.random() * 100 > config.sessionSampleRate) {
        return; // Not sampled
      }

      initialized = true;

      // Start flush timer
      flushTimer = setInterval(flush, config.flushIntervalMs);

      // Track page load
      trackPageLoad();

      // Track errors
      if (config.trackErrors) {
        setupErrorTracking();
      }

      // Track network requests
      if (config.trackNetworkRequests) {
        setupNetworkTracking();
      }

      // Track user actions
      if (config.trackUserActions) {
        setupActionTracking();
      }

      // Track page visibility changes
      setupVisibilityTracking();

      // Flush on page unload
      window.addEventListener('beforeunload', flush);

      // Track SPA navigation
      setupSPATracking();

      console.log(`[FluxxRUM] Initialized (app=${config.applicationId}, session=${sessionId.slice(0, 8)})`);
    },

    /**
     * Set user identity for session correlation.
     */
    setUser(userId, email, attributes) {
      config.userId = userId;
      config.userEmail = email;
      if (attributes) {
        Object.assign(config.customAttributes, attributes);
      }
    },

    /**
     * Track a custom action/event.
     */
    addAction(name, attributes) {
      addEvent('action', {
        action_name: name,
        action_type: 'custom',
        ...attributes,
      });
    },

    /**
     * Track a custom error.
     */
    addError(error, context) {
      const errorData = error instanceof Error
        ? { message: error.message, stack: error.stack, type: error.name }
        : { message: String(error), type: 'CustomError' };

      addEvent('error', {
        error_message: errorData.message,
        error_type: errorData.type,
        error_stack: errorData.stack || '',
        ...context,
      });
    },

    /**
     * Track a custom timing measurement.
     */
    addTiming(name, durationMs) {
      addEvent('timing', {
        timing_name: name,
        timing_duration_ms: durationMs,
      });
    },

    /**
     * Start a new view (for SPA navigation).
     */
    startView(name, url) {
      pageViewId = generateId();
      addEvent('view', {
        view_name: name,
        view_url: url || window.location.href,
      });
    },

    /**
     * Get current session ID.
     */
    getSessionId() {
      return sessionId;
    },

    /**
     * Force flush buffered events.
     */
    flush() {
      flush();
    },

    /**
     * Stop the SDK.
     */
    stop() {
      if (flushTimer) clearInterval(flushTimer);
      flush();
      initialized = false;
    },
  };

  // ── Web Vitals ──

  function trackPageLoad() {
    // Wait for load event to get complete timing
    if (document.readyState === 'complete') {
      capturePerformanceMetrics();
    } else {
      window.addEventListener('load', () => {
        // Delay slightly to ensure all timing entries are available
        setTimeout(capturePerformanceMetrics, 100);
      });
    }

    addEvent('view', {
      view_name: document.title,
      view_url: window.location.href,
      view_referrer: document.referrer,
    });
  }

  function capturePerformanceMetrics() {
    const timing = performance.timing || {};
    const nav = performance.getEntriesByType('navigation')[0] || {};

    // Navigation timing
    const metrics = {
      ttfb_ms: (nav.responseStart || timing.responseStart) - (nav.requestStart || timing.navigationStart),
      dom_content_loaded_ms: (nav.domContentLoadedEventEnd || timing.domContentLoadedEventEnd) - (nav.navigationStart || timing.navigationStart),
      load_event_ms: (nav.loadEventEnd || timing.loadEventEnd) - (nav.navigationStart || timing.navigationStart),
      dom_interactive_ms: (nav.domInteractive || timing.domInteractive) - (nav.navigationStart || timing.navigationStart),
    };

    // Core Web Vitals via PerformanceObserver
    try {
      // LCP (Largest Contentful Paint)
      new PerformanceObserver((list) => {
        const entries = list.getEntries();
        const lcp = entries[entries.length - 1];
        if (lcp) {
          addEvent('vital', { vital_name: 'LCP', vital_value: lcp.startTime, vital_rating: lcp.startTime < 2500 ? 'good' : lcp.startTime < 4000 ? 'needs-improvement' : 'poor' });
        }
      }).observe({ type: 'largest-contentful-paint', buffered: true });

      // FID (First Input Delay)
      new PerformanceObserver((list) => {
        const entries = list.getEntries();
        if (entries.length > 0) {
          const fid = entries[0].processingStart - entries[0].startTime;
          addEvent('vital', { vital_name: 'FID', vital_value: fid, vital_rating: fid < 100 ? 'good' : fid < 300 ? 'needs-improvement' : 'poor' });
        }
      }).observe({ type: 'first-input', buffered: true });

      // CLS (Cumulative Layout Shift)
      let clsValue = 0;
      new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (!entry.hadRecentInput) clsValue += entry.value;
        }
        addEvent('vital', { vital_name: 'CLS', vital_value: clsValue, vital_rating: clsValue < 0.1 ? 'good' : clsValue < 0.25 ? 'needs-improvement' : 'poor' });
      }).observe({ type: 'layout-shift', buffered: true });

      // FCP (First Contentful Paint)
      new PerformanceObserver((list) => {
        const entries = list.getEntries();
        if (entries.length > 0) {
          addEvent('vital', { vital_name: 'FCP', vital_value: entries[0].startTime, vital_rating: entries[0].startTime < 1800 ? 'good' : entries[0].startTime < 3000 ? 'needs-improvement' : 'poor' });
        }
      }).observe({ type: 'paint', buffered: true });
    } catch (e) {
      // PerformanceObserver not supported
    }

    addEvent('page_load', metrics);

    // Track resources if enabled
    if (config.trackResources) {
      const resources = performance.getEntriesByType('resource');
      for (const res of resources.slice(0, 50)) {
        addEvent('resource', {
          resource_url: res.name,
          resource_type: res.initiatorType,
          resource_duration_ms: res.duration,
          resource_size: res.transferSize || 0,
        });
      }
    }
  }

  // ── Error Tracking ──

  function setupErrorTracking() {
    // Uncaught errors
    window.addEventListener('error', (event) => {
      addEvent('error', {
        error_message: event.message,
        error_type: 'UncaughtError',
        error_stack: event.error ? event.error.stack : '',
        error_filename: event.filename,
        error_lineno: event.lineno,
        error_colno: event.colno,
      });
    });

    // Unhandled promise rejections
    window.addEventListener('unhandledrejection', (event) => {
      const reason = event.reason;
      addEvent('error', {
        error_message: reason instanceof Error ? reason.message : String(reason),
        error_type: 'UnhandledRejection',
        error_stack: reason instanceof Error ? reason.stack : '',
      });
    });

    // Console errors
    const originalError = console.error;
    console.error = function (...args) {
      addEvent('error', {
        error_message: args.map(String).join(' '),
        error_type: 'ConsoleError',
      });
      originalError.apply(console, args);
    };
  }

  // ── Network Tracking ──

  function setupNetworkTracking() {
    // Intercept fetch
    const originalFetch = window.fetch;
    window.fetch = function (input, init) {
      const url = typeof input === 'string' ? input : input.url;
      const method = (init && init.method) || 'GET';
      const start = performance.now();

      return originalFetch.apply(this, arguments).then(
        (response) => {
          addEvent('network', {
            request_url: url,
            request_method: method,
            response_status: response.status,
            duration_ms: performance.now() - start,
            request_type: 'fetch',
          });
          return response;
        },
        (error) => {
          addEvent('network', {
            request_url: url,
            request_method: method,
            response_status: 0,
            duration_ms: performance.now() - start,
            request_type: 'fetch',
            error: error.message,
          });
          throw error;
        }
      );
    };

    // Intercept XMLHttpRequest
    const originalOpen = XMLHttpRequest.prototype.open;
    const originalSend = XMLHttpRequest.prototype.send;

    XMLHttpRequest.prototype.open = function (method, url) {
      this._fluxxMethod = method;
      this._fluxxUrl = url;
      return originalOpen.apply(this, arguments);
    };

    XMLHttpRequest.prototype.send = function () {
      const start = performance.now();
      this.addEventListener('loadend', () => {
        addEvent('network', {
          request_url: this._fluxxUrl,
          request_method: this._fluxxMethod,
          response_status: this.status,
          duration_ms: performance.now() - start,
          request_type: 'xhr',
        });
      });
      return originalSend.apply(this, arguments);
    };
  }

  // ── User Action Tracking ──

  function setupActionTracking() {
    document.addEventListener('click', (event) => {
      const target = event.target;
      const tagName = target.tagName;
      const text = (target.textContent || '').trim().slice(0, 100);
      const id = target.id;
      const className = target.className;

      if (['A', 'BUTTON', 'INPUT', 'SELECT'].includes(tagName)) {
        addEvent('action', {
          action_name: text || id || tagName,
          action_type: 'click',
          action_target: `${tagName}${id ? '#' + id : ''}${className ? '.' + String(className).split(' ')[0] : ''}`,
        });
      }
    }, { passive: true, capture: true });
  }

  // ── Visibility Tracking ──

  function setupVisibilityTracking() {
    document.addEventListener('visibilitychange', () => {
      addEvent('visibility', {
        visibility_state: document.visibilityState,
        session_duration_ms: Date.now() - sessionStart,
      });
    });
  }

  // ── SPA Navigation ──

  function setupSPATracking() {
    // History API
    const originalPushState = history.pushState;
    history.pushState = function () {
      originalPushState.apply(this, arguments);
      onNavigate();
    };

    const originalReplaceState = history.replaceState;
    history.replaceState = function () {
      originalReplaceState.apply(this, arguments);
      onNavigate();
    };

    window.addEventListener('popstate', onNavigate);
  }

  function onNavigate() {
    pageViewId = generateId();
    addEvent('view', {
      view_name: document.title,
      view_url: window.location.href,
    });
  }

  // ── Event Buffer ──

  function addEvent(type, data) {
    if (!initialized || eventCount >= config.maxEventsPerSession) return;

    eventBuffer.push({
      type,
      timestamp_us: Date.now() * 1000,
      session_id: sessionId,
      page_view_id: pageViewId,
      application_id: config.applicationId,
      environment: config.environment,
      app_version: config.version,
      user_id: config.userId,
      user_email: config.userEmail,
      url: window.location.href,
      user_agent: navigator.userAgent,
      screen_width: window.screen.width,
      screen_height: window.screen.height,
      viewport_width: window.innerWidth,
      viewport_height: window.innerHeight,
      ...data,
      ...config.customAttributes,
    });

    eventCount++;

    // Auto-flush if buffer is large
    if (eventBuffer.length >= 50) {
      flush();
    }
  }

  function flush() {
    if (eventBuffer.length === 0) return;

    const events = eventBuffer.splice(0);

    // Send via Beacon API (survives page unload)
    if (navigator.sendBeacon) {
      const payload = JSON.stringify(events);
      navigator.sendBeacon(config.endpoint + '/v1/rum', payload);
    } else {
      // Fallback to fetch
      try {
        fetch(config.endpoint + '/v1/rum', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(events),
          keepalive: true,
        }).catch(() => {});
      } catch (e) {}
    }
  }

  // ── Utils ──

  function generateId() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = (Math.random() * 16) | 0;
      return (c === 'x' ? r : (r & 0x3) | 0x8).toString(16);
    });
  }

  // ── Export ──
  global.FluxxRUM = FluxxRUM;

})(typeof window !== 'undefined' ? window : globalThis);
