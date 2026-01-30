# Product Blueprint: HiAgent Backend Template

*Generated: 2026-01-26*

## Executive Summary

A production-ready Python backend template that accelerates AI product development by providing pre-integrated HiAgent SDK, database, observability, and extensible architecture.

Every AI product built on HiAgent requires the same foundational work—SDK integration, database setup, scalability patterns, and configuration management. This template eliminates that repetitive setup, providing a standardized, extensible foundation that teams can clone and immediately focus on business logic. Built for on-prem deployment with Docker, it includes everything from async-first API handling to event-driven architecture, enabling faster experimentation and consistent quality across all HiAgent-based projects.

**Key Numbers:**
- 3 user personas (Backend Developer, Platform/DevOps Engineer, Product Configurator)
- 28 features across 7 categories
- 5 data entities (AgentConfig, RequestLog, Event, User, AgentTestRun)
- 6 integrations

**Goal:** Enable developers to go from template clone to working HiAgent-connected API in under a day, with a dashboard for non-technical users to configure and monitor agents.

---

## 1. Vision

### One-Liner
HiAgent Backend Template enables rapid AI product development with standardized infrastructure and dashboard-driven agent configuration.

### Problem Statement
Every AI product built on HiAgent requires the same foundational work: SDK integration, database setup, scalability patterns, and agent configuration. With HiAgent newly deployed, each project repeats this setup, creating inconsistent codebases and slowing experimentation. No reusable template exists to standardize this foundation.

### Value Proposition
This template provides a production-ready Python backend with HiAgent SDK integration, PostgreSQL, and a dashboard for agent/workflow selection. Teams skip infrastructure setup and jump straight to business logic. Event-driven architecture ensures extensibility as requirements evolve.

### Success Criteria
1. New AI projects reach business logic development within 1 day (vs. current multi-day setup)
2. 100% of new HiAgent-based projects use this template as their foundation
3. Zero manual SDK key configuration required (dashboard handles it)
4. Template supports 3+ concurrent production deployments without modification
5. At least 2 different business domains successfully extend the template

---

## 2. Target Users

### Backend Developer (Primary)

**Description:** Developers who build AI products using the template, implement business logic, and extend the core functionality. Mixed skill levels from junior to senior.

**Goals:**
- Primary: Rapidly build AI products without reinventing infrastructure
- Secondary: Follow best practices, write maintainable code, extend for business needs

**Pain Points:**
- Starting from scratch or copy-pasting code for each project
- Inconsistent setups across projects
- Uncertainty about best practices

**Must-Haves:**
- Pre-integrated HiAgent SDK
- Extensible architecture
- Instant working setup
- Proven scalability patterns

**Nice-to-Haves:**
- Example business logic implementations
- Testing utilities

**Anti-Requirements:**
- Rigid/opinionated frameworks that can't be customized
- Poor documentation

---

### Platform/DevOps Engineer (Secondary)

**Description:** Engineers who deploy, maintain, and configure the template infrastructure on on-prem systems.

**Goals:**
- Primary: Deploy and maintain AI products reliably on on-prem infrastructure
- Secondary: Monitor system health, manage configurations across environments, ensure security compliance

**Pain Points:**
- Each project deploys differently
- Missing observability
- Unclear resource requirements
- Security gaps

**Must-Haves:**
- Easy deployment scripts
- Built-in observability (logging/metrics/health)
- Environment-based configuration
- Security compliance

**Nice-to-Haves:**
- Resource sizing guidelines
- Runbook templates

**Anti-Requirements:**
- Cloud-dependent features
- Manual deployment steps

---

### Product Configurator

**Description:** Product managers and business users who configure which agents/workflows a product uses through a visual dashboard, without requiring developer involvement.

**Goals:**
- Primary: Configure which agents/workflows a product uses without developer involvement
- Secondary: Understand available agents, preview agent behavior, monitor usage

**Pain Points:**
- Need developers for all agent configuration changes
- Can't see what's available in HiAgent
- No visibility into agent performance or usage
- Configuration changes require code deployments

**Must-Haves:**
- Visual agent selection interface
- Configuration UI with no code required
- Agent/workflow browser showing available options
- Basic usage statistics

**Nice-to-Haves:**
- Agent preview/testing before enabling
- A/B testing capabilities
- Configuration version history

**Anti-Requirements:**
- Code-based configuration
- Technical jargon in UI
- Complex deployment processes for config changes

---

## 3. Features

### All Features (28 Total)

| ID | Category | Feature | Description |
|----|----------|---------|-------------|
| F1 | Core | HiAgent SDK integration | Pre-integrated HiAgent Python SDK with connection pooling |
| F2 | Core | FastAPI framework | FastAPI-based REST API framework |
| F3 | Core | PostgreSQL + migrations | PostgreSQL database with Alembic migrations |
| F4 | Core | Environment config | Environment-based configuration (dev/staging/prod) |
| F5 | Core | Async-first architecture | Async-first architecture for concurrent operations |
| F6 | Scalability | Worker pattern | Worker pattern for background agent processing |
| F7 | Scalability | Connection pooling | Connection pooling and request queuing |
| F8 | Scalability | Horizontal scaling | Horizontal scaling support (stateless design) |
| F9 | Observability | Structured logging | Structured logging (JSON format) |
| F10 | Observability | Metrics collection | Metrics collection (Prometheus-compatible) |
| F11 | Observability | Health checks | Health check endpoints |
| F12 | Observability | Request tracing | Request tracing with correlation IDs |
| F13 | Extensibility | Event bus | Event bus for internal communication |
| F14 | Extensibility | Plugin system | Plugin system for business logic modules |
| F15 | Extensibility | Module structure | Standardized module structure for extensions |
| F16 | DevOps | Docker | Docker containerization |
| F17 | DevOps | Deployment scripts | Deployment scripts for on-prem |
| F18 | DevOps | Secrets handling | Configuration management (secrets handling) |
| F19 | DX | Scaffolding CLI | Project scaffolding CLI |
| F20 | DX | Example module | Example business logic module |
| F21 | DX | Documentation | Comprehensive documentation |
| F22 | DX | Testing utilities | Testing utilities and fixtures |
| F23 | Dashboard | Agent browser | Visual interface to browse and manage agent configurations |
| F24 | Dashboard | Configuration UI | No-code interface for configuring agent parameters and settings |
| F25 | Dashboard | Agent preview/testing | Test agent behavior before enabling in production |
| F26 | Dashboard | Usage analytics | Real-time dashboard showing agent usage, performance metrics, and trends via WebSocket/SSE |
| F27 | Dashboard | Resource guidelines | Documentation and UI hints for resource sizing and capacity planning |
| F28 | Dashboard | Runbook integration | Operational runbooks accessible from dashboard for common tasks |

### Feature Categories Summary

| Category | Count | Features |
|----------|-------|----------|
| Core | 5 | F1-F5 |
| Scalability | 3 | F6-F8 |
| Observability | 4 | F9-F12 |
| Extensibility | 3 | F13-F15 |
| DevOps | 3 | F16-F18 |
| DX | 4 | F19-F22 |
| Dashboard | 6 | F23-F28 |

---

## 4. User Flows

### Flow 1: First-Time Setup

**Purpose:** Developer gets a working local environment with API responding and HiAgent connected

**Entry:** Developer decides to use template for new AI project

**Steps:**
1. Clone template or run scaffolding CLI (F19)
2. Install dependencies (F2)
3. Copy .env.example, configure HiAgent URL/credentials (F4, F18)
4. Run database migrations (F3)
5. Start application (F2, F5)
6. Test health check endpoint (F11)
7. Test HiAgent connection (F1)

**Exit:** Developer has working local environment, API responds, HiAgent connected

**Edge Cases:**
- HiAgent URL wrong/unreachable → Health check fails with clear error; docs explain connectivity verification
- Database not running → Migration command shows clear error; docs explain DB setup
- Missing environment variables → Application fails fast with list of missing required vars

---

### Flow 2: Add Business Logic

**Purpose:** Developer creates and deploys a new business logic module

**Entry:** Developer has requirement for new business functionality

**Steps:**
1. Create module directory following structure (F15, F20)
2. Define database models if needed (F3)
3. Create API endpoints (F2)
4. Implement business logic (F5)
5. Subscribe to/publish events (F13)
6. Write tests (F22)
7. Run and verify (F11)

**Exit:** New business logic module deployed and working

**Edge Cases:**
- Module structure non-standard → Linter/validator warns developer
- Database migration conflict → Alembic detects conflict; docs explain resolution

---

### Flow 3: Integrate HiAgent Agent

**Purpose:** Developer successfully calls HiAgent agent from business logic

**Entry:** Developer needs AI capability in business logic

**Steps:**
1. Check available AgentConfigs in database or dashboard (F1)
2. Use agent_config_id or name to lookup config (F1, F4)
3. Call agent service with agent_config_id, query, and user context (F1, F5, F7)
4. Service resolves app_key from AgentConfig → Agent.ainit(svc, app_key, user_id=email)
5. Handle async response from agent.ainvoke({"query": ...}) (F5)
6. Implement error handling (F9, F12)
7. Test agent integration (F22)

**Exit:** Business logic successfully calls HiAgent agent and handles response

**Edge Cases:**
- AgentConfig not found → AgentConfigNotFound error with available configs
- Agent times out → Configurable timeout; retry logic with backoff; event for monitoring
- SDK auth fails → Check VOLC_ACCESSKEY/VOLC_SECRETKEY in environment
- Rate limited by HiAgent → Request queuing handles backpressure; metrics track queue depth

---

### Flow 4: Configure Agent via Dashboard

**Purpose:** Admin/Configurator adds a new agent configuration so users can integrate easily

**Entry:** Admin needs to make a HiAgent agent available through the backend

**Steps:**
1. Log into dashboard (F24)
2. Navigate to agent configuration section (F23)
3. Click "Add Agent Configuration"
4. Enter agent details: name, app_key (from HiAgent platform), variables (F24)
5. Optionally test the agent with sample input (F25)
6. Save configuration to database
7. Verify agent appears in available configurations
8. Users can now integrate using the config name/ID

**Exit:** Agent is configured and available for integration

**Edge Cases:**
- Invalid app_key → Validation fails when testing; clear error message
- Duplicate name → Error shown; suggest unique name
- Agent test fails → Error displayed with details (timeout, SDK error, etc.)

> **Note:** Agents are created on the HiAgent platform. This dashboard manages which agents are available through this backend and their configuration.

---

### Flow 5: Monitor Agent Usage

**Purpose:** Product Configurator or DevOps reviews agent performance and usage

**Entry:** User wants to understand how agents are performing

**Steps:**
1. Log into dashboard (F24)
2. Navigate to analytics section (F26)
3. View usage overview (requests, tokens, response times)
4. Filter by agent, time range, or status
5. Drill down into specific agent metrics
6. Identify issues or optimization opportunities
7. Access runbook if action needed (F28)

**Exit:** User understands agent usage patterns and performance

**Edge Cases:**
- No data for time range → Dashboard shows empty state with suggestion
- Metrics delayed → Warning banner indicates data freshness

---

### Flow 6: Deploy with Resource Planning

**Purpose:** DevOps deploys template with appropriate resource allocation

**Entry:** DevOps preparing for production deployment

**Steps:**
1. Review resource guidelines documentation (F27)
2. Access resource calculator in dashboard (F27)
3. Input expected load parameters
4. Receive resource recommendations
5. Configure deployment with recommended resources (F17)
6. Deploy application (F16)
7. Monitor initial performance (F26)
8. Adjust resources based on actual usage

**Exit:** Application deployed with appropriate resources

**Edge Cases:**
- Unexpected load spike → Runbook provides scaling guidance (F28)
- Resource constraints → Guidelines suggest minimum viable configuration

---

## 5. Data Model

### Entities

**AgentConfig**
> Agents are created on the HiAgent platform. AgentConfig stores the backend's registry of which agents are available, making it easy for users to integrate without dealing with HiAgent keys directly.

- name: Human-readable identifier
- app_key: HiAgent application key for SDK calls
- config_type: Type of config (agent, workflow, retriever, tool)
- workspace_id: HiAgent workspace identifier (for workflows/tools/knowledge)
- workflow_id: Workflow ID (if config_type=workflow)
- dataset_ids: Knowledge base dataset IDs (if config_type=retriever)
- tool_id: Tool ID (if config_type=tool)
- variables: Default input variables for the agent/workflow
- enabled: Whether this config is active
- created_by: User who created the config
- updated_by: User who last modified the config

**RequestLog**
- correlation_id: Unique request identifier for tracing
- endpoint: API endpoint called
- agent_config_id: Which agent config was invoked
- user_id: User who made the request (email from JWT)
- query: User's input message
- status: Success/failure
- duration_ms: Request duration
- tokens_in: Input tokens consumed
- tokens_out: Output tokens generated
- conversation_id: HiAgent conversation ID (for multi-turn)
- timestamp: When request occurred

**Event**
- event_type: Category of event
- payload: Event data
- source: What triggered the event
- correlation_id: Links to request
- status: Pending/processed/failed
- timestamp: When event was created

**User** (Dashboard)
- username: Unique identifier for login
- email: User email address
- password_hash: Hashed password for authentication
- role: User role (admin, configurator, viewer)
- last_login: Last successful login timestamp
- active: Whether user account is active

**Role Permissions:**
| Role | User Mgmt | Agent Config | Agent Test | Analytics | View Only |
|------|-----------|--------------|------------|-----------|-----------|
| Admin | ✓ | ✓ | ✓ | ✓ | ✓ |
| Configurator | - | ✓ | ✓ | ✓ | ✓ |
| Viewer | - | - | - | ✓ | ✓ |

**AgentTestRun** (Dashboard)
- agent_config_id: Which agent configuration was tested
- test_input: Input provided for the test
- test_output: Response received from agent
- status: Success/failure/timeout
- duration_ms: Test execution time
- tokens_in: Input tokens consumed
- tokens_out: Output tokens generated
- tested_by: User who ran the test
- timestamp: When test was executed

### Relationships

```
RequestLog ──references──> AgentConfig (logs which agent was invoked)
RequestLog ──made_by──> User (who made the request)
Event ──references──> RequestLog (events link back to originating request)
AgentConfig ──created_by──> User (tracks who configured the agent)
AgentTestRun ──references──> AgentConfig (test run for specific config)
AgentTestRun ──tested_by──> User (who executed the test)
```

### Lifecycle

| Entity | Created | Updated | Deleted |
|--------|---------|---------|---------|
| AgentConfig | User configures new agent via dashboard | Settings changed | Agent removed from system |
| RequestLog | Every API request | Never (immutable) | Retention policy (90 days) |
| Event | Event published | Status changes | Retention policy (30 days) |
| User | Admin creates account | Profile/permissions changed | Account deactivated |
| AgentTestRun | User runs agent test | Never (immutable) | Retention policy (30 days) |

---

## 6. Integrations

| Service | Category | Purpose | Provider | Criticality |
|---------|----------|---------|----------|-------------|
| HiAgent | AI/ML Platform | Agent execution | HiAgent (internal) | Critical |
| PostgreSQL | Database | Data persistence | PostgreSQL (self-hosted) | Critical |
| Redis | Task Queue | Background processing | Redis (self-hosted) | Critical |
| Prometheus | Observability | Metrics collection | Prometheus (self-hosted) | Nice-to-have |
| Log Aggregation | Observability | Log collection | ELK/Loki (external) | Nice-to-have |
| Dashboard Frontend | UI | Configuration and monitoring UI | React/Vue (bundled) | Critical |

### Integration Details

**HiAgent**
- **SDK:** `hiagent-api` + `hiagent-components` from [volcengine/hiagent-python-sdk](https://github.com/volcengine/hiagent-python-sdk)
- **Python:** >= 3.10 required
- **Dependencies:** pydantic >= 2.11.5, httpx, volcengine, tenacity >= 9.1.2
- **Auth:** Volcano Engine credentials - SDK reads directly from environment:
  - `VOLC_ACCESSKEY` (no underscore) - SDK reads automatically
  - `VOLC_SECRETKEY` (no underscore) - SDK reads automatically
- **Endpoints:** Two required environment variables:
  - `HIAGENT_TOP_ENDPOINT` - Platform API (e.g., `https://open.volcengineapi.com`)
  - `HIAGENT_APP_BASE_URL` - Application API (e.g., `https://hia.volcenginepaas.com/api/proxy/api/v1`)
- **Agent Registry:** `HIAGENT_AGENTS` - JSON mapping agent_id → app_key
- **Services:** ChatService (agents), WorkflowService, KnowledgebaseService (RAG), ToolService
- **Initialization Pattern:** Singleton ChatService initialized at startup:
  ```python
  svc = ChatService(endpoint=HIAGENT_TOP_ENDPOINT, region="cn-north-1")
  svc.set_app_base_url(HIAGENT_APP_BASE_URL)
  ```
- **Per-Request:** Agent.ainit(svc, app_key, user_id=email) → agent.ainvoke({"query": ...})
- **Data In:** app_key (from registry), user_id (from JWT email), query, conversation_id
- **Data Out:** Agent response (streaming or blocking), conversation_id, token usage
- **Trigger:** API request from business logic
- **Fallback:** Return error with correlation ID, retry with tenacity, circuit breaker pattern

**PostgreSQL**
- **Data In:** All entity data (AgentConfig, RequestLog, Event)
- **Data Out:** Query results
- **Trigger:** Application requests
- **Fallback:** Health check fails; application won't start without DB

**Redis**
- **Data In:** Task payloads for worker processing
- **Data Out:** Task results, status
- **Trigger:** When agent call needs async processing
- **Fallback:** Synchronous processing; degrade gracefully

**Prometheus**
- **Data In:** N/A (scrape model)
- **Data Out:** Metrics at /metrics endpoint
- **Trigger:** Prometheus scrapes periodically
- **Fallback:** Metrics not collected (app continues working)

**Log Aggregation**
- **Data In:** Structured JSON logs to stdout
- **Data Out:** N/A
- **Trigger:** Every log statement
- **Fallback:** Logs go to stdout only

**Dashboard Frontend**
- **Technology:** React 18+ with TypeScript
- **UI Framework:** shadcn/ui + Tailwind CSS
- **Build:** Bundled with backend, served by FastAPI (static files)
- **Data In:** User interactions, configuration changes
- **Data Out:** API calls to backend endpoints
- **Auth:** JWT tokens (stateless, horizontal scaling friendly)
- **Real-time:** WebSocket or SSE for live analytics updates
- **Features Required:**
  - Agent browser with search/filter
  - Configuration forms with validation
  - Real-time test execution UI
  - Live analytics charts (using Recharts or similar)
  - Responsive design for various screen sizes
- **User Roles:**
  - Admin: Full access (user management, all configurations)
  - Configurator: Agent configuration and testing
  - Viewer: Read-only access to dashboard and analytics
- **Fallback:** Backend API remains functional without dashboard

---

## 7. Constraints

### Performance
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| API response < 500ms for standard endpoints | Standard API expectations | Must |
| Support moderate concurrent load | Multiple projects using template | Should |

### Security
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Basic auth, HTTPS, input validation | Baseline security | Must |
| Secrets not in code (env vars/config) | Security best practice | Must |

### Platform
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Docker container deployment | On-prem infrastructure standard | Must |
| Stateless design for horizontal scaling | Each deployment scales independently | Must |

### Extensibility
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Modules can override/extend defaults | Prevent "too rigid" concern | Must |
| Plugin architecture for business logic | Allow customization without forking | Should |

### Resilience
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Graceful degradation when HiAgent down | Queue requests, show errors, retry | Must |
| Circuit breaker for external calls | Prevent cascade failures | Should |

### Developer Experience
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Comprehensive out-of-box features | Address "missing features" concern | Must |
| Clear documentation and examples | Support adoption | Must |

### Flexibility
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Architecture accommodates change | Build for evolution | Should |

### Dashboard/UI
| Requirement | Rationale | Priority |
|-------------|-----------|----------|
| Dashboard loads < 3 seconds | User experience standard | Must |
| Works on modern browsers (Chrome, Firefox, Safari, Edge) | Cross-browser support | Must |
| Responsive design (desktop and tablet) | Flexibility for different devices | Should |
| Accessible (WCAG 2.1 AA) | Inclusive design | Should |
| No technical jargon in UI | Product Configurator persona needs | Must |
| Real-time feedback for agent tests | Interactive testing experience | Must |
| Role-based access control | Security and permissions | Must |

---

## 8. Open Questions

### All Resolved

- [x] **HiAgent SDK:** Use official `hiagent-api` + `hiagent-components` from [volcengine/hiagent-python-sdk](https://github.com/volcengine/hiagent-python-sdk)
- [x] **Authentication:** Volcano Engine credentials via environment variables (`VOLC_ACCESSKEY`, `VOLC_SECRETKEY`) - SDK reads directly, no underscore!
- [x] **Endpoints:** Two required: `HIAGENT_TOP_ENDPOINT` + `HIAGENT_APP_BASE_URL`
- [x] **Agent Config:** Database-based AgentConfig stores app_key and settings; agents created on HiAgent platform, config stored in our database for easy integration
- [x] **Python version:** >= 3.10 required by SDK
- [x] **Redis topology:** Single instance default, Sentinel for production HA (see Appendix C)
- [x] **Log retention:** 90d hot / 1y cold for RequestLog, 30d for Events (see Appendix D)
- [x] **Metrics:** HiAgent-specific + task queue + event bus metrics (see Appendix E)
- [x] **user_id:** Logged-in user's email from JWT (required for SDK calls)

### SDK Integration Notes (2026-01-28)

Key SDK integration patterns based on actual implementation:

1. **Environment Variables:** SDK reads `VOLC_ACCESSKEY`/`VOLC_SECRETKEY` directly (no underscore), plus `HIAGENT_TOP_ENDPOINT` and `HIAGENT_APP_BASE_URL`

2. **AgentConfig in Database:** Stores HiAgent app_key, name, variables so users can integrate without dealing with HiAgent credentials directly

3. **Dashboard:** Allows admins to create/manage AgentConfig entries, mapping friendly names to HiAgent app_keys

---

## Appendix A: Environment Variables

```bash
# ============================================
# Platform Auth (SDK reads directly - no underscore!)
# ============================================
VOLC_ACCESSKEY=your_access_key
VOLC_SECRETKEY=your_secret_key

# ============================================
# HiAgent Endpoints (both required)
# ============================================
HIAGENT_TOP_ENDPOINT=https://open.volcengineapi.com
HIAGENT_APP_BASE_URL=https://hia.volcenginepaas.com/api/proxy/api/v1

# ============================================
# Database
# ============================================
DATABASE_URL=postgresql+asyncpg://user:pass@localhost:5432/hiagent

# ============================================
# Dashboard Auth (our backend, not HiAgent)
# ============================================
HIAGENT_JWT_SECRET=your_jwt_secret_here
HIAGENT_ADMIN_USERNAME=admin
HIAGENT_ADMIN_PASSWORD=your_admin_password
HIAGENT_ADMIN_EMAIL=admin@company.com
```

> **Important:** The SDK reads `VOLC_ACCESSKEY` and `VOLC_SECRETKEY` directly from the environment (no underscore). Do NOT use `VOLC_ACCESS_KEY` or `VOLC_SECRET_KEY`.

> **Note:** Agent configurations (app_key, name, variables) are stored in the database via AgentConfig entity, not in environment variables.

## Appendix B: HiAgent SDK Quick Reference

| Feature | Sync | Async |
|---------|------|-------|
| Agent invoke | `agent.invoke()` | `await agent.ainvoke()` |
| Agent stream | `agent.stream()` | `agent.astream()` |
| Workflow | `workflow.invoke()` | `await workflow.ainvoke()` |
| RAG Retriever | `retriever.invoke()` | `await retriever.ainvoke()` |
| Tool | `tool.invoke()` | `await tool.ainvoke()` |

**Key Services:**
- `ChatService` - Agent conversations (blocking + streaming)
- `WorkflowService` - Workflow execution (async polling or blocking)
- `KnowledgebaseService` - RAG retrieval (knowledge, QA, terminology)
- `ToolService` - External tool execution

**Streaming Events:** `message`, `tool_message`, `think_message`, `agent_thought`, `message_cost`, `suggestion`, `interrupted`

---

## Appendix C: Redis Deployment Strategy

### Default: Single Instance (Development/Staging)

```yaml
redis:
  mode: standalone
  host: ${REDIS_HOST:-localhost}
  port: ${REDIS_PORT:-6379}
  db: ${REDIS_DB:-0}
```

### Production: Sentinel (High Availability)

```yaml
redis:
  mode: sentinel
  sentinels:
    - host: sentinel1
      port: 26379
    - host: sentinel2
      port: 26379
    - host: sentinel3
      port: 26379
  master_name: mymaster
```

**Rationale:**
- Single instance is simple for development and low-traffic deployments
- Sentinel provides automatic failover for production HA
- Cluster mode is overkill for task queue use cases
- Template code should abstract Redis mode so switching is config-only

---

## Appendix D: Data Retention Policies

| Data Type | Hot Retention | Cold/Archive | Rationale |
|-----------|---------------|--------------|-----------|
| **RequestLog** | 90 days | 1 year | API audit trail, compliance |
| **Event** | 30 days | None | Operational data, not audit-critical |
| **Application Logs** | 30 days | 90 days | Debugging, incident response |
| **Metrics** | 15 days (high-res) | 1 year (aggregated) | Performance trending |

### Configuration

```bash
# Environment variables for retention
RETENTION_REQUEST_LOG_HOT_DAYS=90
RETENTION_REQUEST_LOG_COLD_DAYS=365
RETENTION_EVENT_DAYS=30
```

### Implementation

- Scheduled job runs daily to enforce retention
- Hot data in primary PostgreSQL tables
- Cold data moved to archive tables or external storage
- Each deployment can override defaults via environment variables

---

## Appendix E: Application Metrics

### Standard HTTP Metrics (from FastAPI middleware)
- `http_requests_total{method, endpoint, status}`
- `http_request_duration_seconds{method, endpoint}`

### HiAgent-Specific Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `hiagent_request_total` | Counter | agent_id, status | Total HiAgent API calls |
| `hiagent_request_duration_seconds` | Histogram | agent_id | Agent call latency |
| `hiagent_tokens_total` | Counter | agent_id, direction | Token consumption (input/output) |
| `hiagent_errors_total` | Counter | agent_id, error_type | Error patterns |

### Task Queue Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `task_queue_depth` | Gauge | queue | Current queue size |
| `task_duration_seconds` | Histogram | task_type | Worker processing time |
| `task_failures_total` | Counter | task_type, reason | Failed tasks |

### Event Bus Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `events_published_total` | Counter | event_type | Events published |
| `events_processed_total` | Counter | event_type, status | Events processed |
| `event_processing_lag_seconds` | Gauge | - | Consumer lag |

### Business Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `conversations_active` | Gauge | - | Active conversations |
| `agent_configs_total` | Gauge | enabled | Configured agents |

---

*Blueprint generated by create-blueprint skill*
