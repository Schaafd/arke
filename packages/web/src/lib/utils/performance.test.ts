import { describe, it, expect, beforeEach } from 'vitest';
import { KeystrokeLatencyTester } from './performance';

describe('KeystrokeLatencyTester', () => {
  let tester: KeystrokeLatencyTester;

  beforeEach(() => {
    tester = new KeystrokeLatencyTester();
  });

  it('should record and calculate latency statistics', () => {
    const latencies = [5, 8, 3, 12, 7, 9, 6, 11, 4, 10];

    latencies.forEach((latency) => tester.recordLatency(latency));

    const results = tester.getResults();

    expect(results.samples).toBe(10);
    expect(results.min).toBe(3);
    expect(results.max).toBe(12);
    expect(results.average).toBeCloseTo(7.5, 1);
    expect(results.p95).toBeLessThanOrEqual(12);
  });

  it('should return zeros for empty samples', () => {
    const results = tester.getResults();

    expect(results.samples).toBe(0);
    expect(results.min).toBe(0);
    expect(results.max).toBe(0);
    expect(results.average).toBe(0);
    expect(results.p95).toBe(0);
  });

  it('should reset measurements', () => {
    tester.recordLatency(5);
    tester.recordLatency(10);
    tester.reset();

    const results = tester.getResults();
    expect(results.samples).toBe(0);
  });

  it('should meet target latency of <10ms on average', () => {
    // Simulate good performance
    const goodLatencies = [3, 4, 5, 6, 7, 8, 4, 5, 6, 7];
    goodLatencies.forEach((l) => tester.recordLatency(l));

    const results = tester.getResults();
    expect(results.average).toBeLessThan(10);
    expect(results.p95).toBeLessThan(10);
  });
});
