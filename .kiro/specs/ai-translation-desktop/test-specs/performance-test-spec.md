# Performance Test Specification: ai-translation-desktop performanceテスト

**Author**: Auto-generated
**Date**: 2025-12-01
**Version**: 1.0

## 1. Overview

### 1.1 Purpose
ai-translation-desktopのパフォーマンスが要件を満たすことを確認する

Example: To verify the performance requirements of the  system and confirm system behavior under high load conditions.

### 1.2 Scope
ai-translation-desktopの主要な処理のパフォーマンス

Example: This test specification covers response time, throughput, and resource utilization for the user management feature.

### 1.3 Testing Tool
- **Tool**: Vitest
- **Version**: 1.0.0

Example:
- **Tool**: Apache JMeter / k6 / Artillery / Locust
- **Version**: 5.6.2 / 0.47.0 / 2.0.0 / 2.15.0

## 2. Test Environment

### 2.1 Software Requirements
- Programming Language: TypeScript 1.0.0
- Application Server:  1.0.0
- Database:  1.0.0
- Load Balancer:  (if applicable)

### 2.2 Hardware Requirements
- Test Client: 
- Application Server: 
- Database Server: 
- Network: 

Example:
- Test Client: 4 vCPU, 8GB RAM
- Application Server: 8 vCPU, 16GB RAM
- Database Server: 16 vCPU, 32GB RAM
- Network: 10Gbps LAN

### 2.3 Test Data
- Data Volume: 
- Data Preparation Script: ``
- Data Cleanup Script: ``

## 3. Performance Requirements

### 3.1 Response Time Requirements

| Endpoint/Function | Expected Response Time (Average) | Maximum Acceptable Response Time (95th Percentile) |
|-------------------|----------------------------------|---------------------------------------------------|
|  |  ms |  ms |
|  |  ms |  ms |

Example:

| Endpoint/Function | Expected Response Time (Average) | Maximum Acceptable Response Time (95th Percentile) |
|-------------------|----------------------------------|---------------------------------------------------|
| GET /api/users | 200 ms | 500 ms |
| POST /api/users | 300 ms | 800 ms |
| GET /api/users/{id} | 100 ms | 300 ms |

### 3.2 Throughput Requirements

| Scenario | Expected Throughput | Minimum Acceptable Throughput |
|----------|---------------------|------------------------------|
|  |  req/sec |  req/sec |
|  |  req/sec |  req/sec |

Example:

| Scenario | Expected Throughput | Minimum Acceptable Throughput |
|----------|---------------------|------------------------------|
| User Registration | 100 req/sec | 80 req/sec |
| User Search | 500 req/sec | 400 req/sec |

### 3.3 Resource Utilization Requirements

| Resource | Maximum Acceptable Usage |
|----------|-------------------------|
| CPU | % |
| Memory | % |
| Disk I/O |  IOPS |
| Network |  Mbps |

Example:

| Resource | Maximum Acceptable Usage |
|----------|-------------------------|
| CPU | 70% |
| Memory | 80% |
| Disk I/O | 5000 IOPS |
| Network | 800 Mbps |

## 4. Test Scenarios

### Test Scenario PT-001: 

**Description**: 

Example: Verify performance of user registration API under normal load conditions.

**Load Pattern**:
- Concurrent Users: 
- Ramp-up Time:  seconds
- Test Duration:  minutes

Example:
- Concurrent Users: 100
- Ramp-up Time: 60 seconds
- Test Duration: 10 minutes

**Test Steps**:
1. 
2. 
3. 

Example:
1. Gradually increase to 100 users over 60 seconds
2. Send POST requests to /api/users (each user)
3. Continue for 10 minutes
4. Measure response time and throughput

**Expected Results**:

Example:
- Average response time: ≤ 300ms
- 95th percentile: ≤ 800ms
- Throughput: ≥ 100 req/sec
- Error rate: ≤ 1%

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

### Test Scenario PT-002: Stress Test - 

**Description**: 

Example: Gradually increase load to identify system limits.

**Load Pattern**:
- Starting Concurrent Users: 
- Final Concurrent Users: 
- Increment Rate:  users/minute
- Test Duration:  minutes

Example:
- Starting Concurrent Users: 50
- Final Concurrent Users: 500
- Increment Rate: 50 users/minute
- Test Duration: 15 minutes

**Expected Results**:

Example:
- Identify the threshold at which system breaks down
- Threshold: Above 400 concurrent users
- Symptoms at breakdown: Response time >5 seconds, Error rate >10%

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

---

### Test Scenario PT-003: Endurance Test (Soak Test) - 

**Description**: 

Example: Verify no memory leaks or performance degradation during extended continuous operation.

**Load Pattern**:
- Concurrent Users:  (constant)
- Test Duration:  hours

Example:
- Concurrent Users: 100 (constant)
- Test Duration: 4 hours

**Expected Results**:

Example:
- Response time does not degrade more than 20% from initial value after 4 hours
- Memory usage does not exceed 80%
- Error rate does not exceed 1%

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

---

### Test Scenario PT-004: Spike Test - 

**Description**: 

Example: Verify system behavior during sudden traffic spikes.

**Load Pattern**:
- Normal Load:  users
- Spike Load:  users
- Spike Duration:  seconds

Example:
- Normal Load: 50 users
- Spike Load: 500 users (10x increase)
- Spike Duration: 60 seconds

**Expected Results**:

Example:
- Error rate ≤ 5% during spike
- System recovers to normal response time after spike ends

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

---

## 5. Test Execution Summary

| ID | Test Name | Status | Executed By | Date | Notes |
|----|-----------|--------|-------------|------|-------|
| PT-001  | |
| PT-002  | |
| PT-003  | |
| PT-004  | |

## 6. Defects Found

| Defect ID | Severity | Description | Affected Components | Status |
|-----------|----------|-------------|---------------------|--------|
| | High/Medium/Low | | | Open/In Progress/Fixed/Closed |

## 7. Sign-off

**Tested By**: _______________
**Date**: _______________
**Approved By**: _______________
**Date**: _______________

---

## Appendix A: Test Environment Setup

### Load Testing Tool Installation

#### Apache JMeter

```bash
# Download and start JMeter
wget https://dlcdn.apache.org//jmeter/binaries/apache-jmeter-5.6.2.tgz
tar -xzf apache-jmeter-5.6.2.tgz
cd apache-jmeter-5.6.2/bin
./jmeter
```

#### k6

```bash
# macOS
brew install k6

# Linux
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6
```

#### Artillery

```bash
npm install -g artillery
```

#### Locust

```bash
# Requires Python environment (Python 3.7+)
pip install locust

# Or in virtual environment
python3 -m venv venv
source venv/bin/activate
pip install locust
```

## Appendix B: Sample Test Scripts

### k6 Script Example

```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 100 },  // Ramp-up
    { duration: '10m', target: 100 }, // Sustained load
    { duration: '1m', target: 0 },    // Ramp-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<800'],  // 95% of requests under 800ms
    http_req_failed: ['rate<0.01'],     // Error rate under 1%
  },
};

export default function () {
  const res = http.post('https://api.example.com/users', JSON.stringify({
    name: 'Test User',
    email: `test-${Date.now()}@example.com`,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });

  check(res, {
    'status is 201': (r) => r.status === 201,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });

  sleep(1);
}
```

### Apache JMeter Test Plan Example

```xml
<?xml version="1.0" encoding="UTF-8"?>
<jmeterTestPlan version="1.2">
  <hashTree>
    <TestPlan guiclass="TestPlanGui" testclass="TestPlan" testname="User API Performance Test">
      <elementProp name="TestPlan.user_defined_variables" elementType="Arguments">
        <collectionProp name="Arguments.arguments"/>
      </elementProp>
    </TestPlan>
    <hashTree>
      <ThreadGroup guiclass="ThreadGroupGui" testclass="ThreadGroup" testname="Users">
        <stringProp name="ThreadGroup.num_threads">100</stringProp>
        <stringProp name="ThreadGroup.ramp_time">60</stringProp>
        <stringProp name="ThreadGroup.duration">600</stringProp>
      </ThreadGroup>
      <hashTree>
        <HTTPSamplerProxy guiclass="HttpTestSampleGui" testclass="HTTPSamplerProxy" testname="POST /api/users">
          <stringProp name="HTTPSampler.domain">api.example.com</stringProp>
          <stringProp name="HTTPSampler.path">/api/users</stringProp>
          <stringProp name="HTTPSampler.method">POST</stringProp>
          <boolProp name="HTTPSampler.use_keepalive">true</boolProp>
        </HTTPSamplerProxy>
      </hashTree>
    </hashTree>
  </hashTree>
</jmeterTestPlan>
```

### Artillery Script Example

```yaml
config:
  target: 'https://api.example.com'
  phases:
    - duration: 60
      arrivalRate: 10
      rampTo: 100
      name: "Ramp up"
    - duration: 600
      arrivalRate: 100
      name: "Sustained load"
  processor: "./custom-functions.js"

scenarios:
  - name: "User Registration"
    flow:
      - post:
          url: "/api/users"
          json:
            name: "Test User"
            email: "test@example.com"
          expect:
            - statusCode: 201
            - contentType: json
```

### Locust Script Example

```python
from locust import HttpUser, task, between

class UserBehavior(HttpUser):
    wait_time = between(1, 3)  # Wait time between users (1-3 seconds)

    @task(3)  # Weight (in this case, runs 3x more frequently)
    def create_user(self):
        """Test user registration API"""
        self.client.post("/api/users", json={
            "name": "Test User",
            "email": f"test-{self.environment.runner.user_count}@example.com"
        })

    @task(1)
    def get_user(self):
        """Test user retrieval API"""
        self.client.get("/api/users/1")

    def on_start(self):
        """Executed when each user starts (e.g., login)"""
        pass
```

**Locust Execution Commands:**

```bash
# Start in web mode (GUI in browser)
locust -f locustfile.py --host=https://api.example.com

# Start in headless mode (CLI)
locust -f locustfile.py --host=https://api.example.com \
  --users 100 --spawn-rate 10 --run-time 10m --headless
```

## Appendix C: Execution Timing

## Phase B (Before Release) - Manual Execution

Performance tests are executed manually before creating a release tag:

1. After PR is merged to main branch
2. Before creating a release tag
3. Run all performance tests in Phase B
4. Proceed to release only after all tests pass

Performance tests are **NOT** executed automatically in CI/CD during PR phase (Phase A) due to long execution times.

## 4. Test Cases

### Test Case PERF-001: 基本的な応答時間テスト

**Description**: 主要な操作の応答時間を測定

**Preconditions**:
- テスト環境が起動している

**Test Steps**:
1. 主要な操作を実行する
2. 応答時間を測定する

**Expected Results**:
- 応答時間が3秒以内（一般的な基準）

**Actual Results**:
[To be filled during test execution]

**Status**: [ ] Pass / [ ] Fail / [ ] Blocked

**Notes**:

---

