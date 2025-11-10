/**
 * Performance testing utilities
 */

export interface LatencyResult {
  average: number;
  min: number;
  max: number;
  p95: number;
  samples: number;
}

/**
 * Measure keystroke latency
 * Returns the time in milliseconds between keydown and the update being reflected
 */
export class KeystrokeLatencyTester {
  private latencies: number[] = [];

  recordLatency(latency: number) {
    this.latencies.push(latency);
  }

  getResults(): LatencyResult {
    if (this.latencies.length === 0) {
      return {
        average: 0,
        min: 0,
        max: 0,
        p95: 0,
        samples: 0,
      };
    }

    const sorted = [...this.latencies].sort((a, b) => a - b);
    const sum = sorted.reduce((acc, val) => acc + val, 0);
    const p95Index = Math.floor(sorted.length * 0.95);

    return {
      average: sum / sorted.length,
      min: sorted[0],
      max: sorted[sorted.length - 1],
      p95: sorted[p95Index],
      samples: sorted.length,
    };
  }

  reset() {
    this.latencies = [];
  }
}

/**
 * Test app startup time
 */
export function measureStartupTime(): number {
  if (typeof performance === 'undefined') {
    return 0;
  }

  const perfData = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
  if (!perfData) {
    return 0;
  }

  // Time from navigation start to DOM content loaded
  return perfData.domContentLoadedEventEnd - perfData.fetchStart;
}

/**
 * Log performance metrics to console
 */
export function logPerformanceMetrics(label: string, metrics: Record<string, number>) {
  console.group(`📊 Performance: ${label}`);
  Object.entries(metrics).forEach(([key, value]) => {
    const formatted = typeof value === 'number' ? value.toFixed(2) : value;
    console.log(`${key}: ${formatted}ms`);
  });
  console.groupEnd();
}
