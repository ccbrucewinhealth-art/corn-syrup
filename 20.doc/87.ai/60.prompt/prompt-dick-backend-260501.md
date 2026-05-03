#### 預設 prompt 的 prefix

清除記憶體與上下文
設定值

    - 目錄
        - 工作目錄: .
        - 程式碼目錄: src/backend
        - 共用程式庫目錄: lib/rust
        - 共用套件目錄: package/rust
        - 執行檔目錄: .
        - 功能規格目錄: 20.doc/48.spec/backend
        - 提示詞目錄： 20.doc/87.ai/60.prompt
    - 工作檔案
        - sdd範本 : {功能規格目錄}/89.sdd-template.md
        - sdd文件 : {功能規格目錄}/25.sdd-backend.md

    - 環境參數檔: .env, 預設使用 {程式碼目錄} 上層 (..) 目錄下的 .env 檔, 如找不到上層，才取用目前目錄下的 .env
    - 環境參數範例檔: env.sample
    - 設定值
        - 環境變數

        - 資料庫連線資訊
            - Host: 192.168.0.25
            - Database: TRC_RData
            - User: TRC201
            - Password: syscom#1
    - 參考系統 : /home/ccbruce/public/github/uptime-kuma/server
    - 源碼轉譯檢查表   : 20.doc/48.spec/80.checklist/80.translate_check-list.md
    - API實做檢查表   : 20.doc/48.spec/80.checklist/82.RestApi_check-list.md

以下目錄不加入分析與考量

    - pycache
    - .venv
    - modules
    - RESUMES
    - spec
    - sql

系統名稱: corn-syrup-backend
開發程式語言: rust
處理過程，使用 繁體中文
系統設定為 i18n , 最少應用 繁體中文，英文，日文，預設為繁體中文

參考規格之提示詞檔案清單 ：

    - N/A

程式碼 檔，約 500 行，應依功能或需求進行切分，必要時應建子目錄, 切分的源碼應進行有意義的命名，不能是 part+no 的格式


所有產出，將結果寫入 {工作目錄}/20.doc/15.resumes/Resume{yyyy}{MM}{dd}-{hh}{mm}{ss}.md
產生 ChangeLog 檔於 {工作目錄}/ChangeLog.md

以為為預設資訊，只載入記憶體中，不進行任何處理

#### 
  參考系統 : /home/ccbruce/public/github/uptime-kuma
  由 參考系統 1對1 將所對應的 源碼 轉成 rust 語言
  進行以下前置功能
     -. 分析 參考系統 產出 
        - {工作目錄}/AGENTS.md 與 {工作目錄}/README.md
        - 依各目錄，在 功能規格目錄 
            - 在 01.Requirements 目錄: 產出 markdown 的 需求文件 (先分成 5 階段)
            - 在 02.Spec         目錄: 產出 相對應 需求文件的 markdown 的 規格文件, 並記錄對應到那個需求
            - 在 03.Plan         目錄: 相對 規格文件 , 產出相關 工作計劃
            - 在 05.Task         目錄: 相對 工作計劃 , 產出相關工作
            - 在 80.checklist    目錄: 
                - 產出 80.translate_check-list.md: 依以下格式產出 轉換檢查表
                    - 格式
                        是否完成("[ ]"), No(編號), DoTranslate(是否進行轉換), Src-Dir(梘源目錄), Src-File(來源檔案), Tar-Dir(目標目錄), Tar-File(目的檔案), Description(描述), Current-status(目前處理況狀)
                    - 說明:
                        - 是否完成: 轉換完，為 [X]
                        - DoTranslate: 如需轉換, YES, 不需轉換: NO (如為 YES 才轉)
                        - Current-status: 轉換前，後說明，記錄在此欄位
                        - 其他請依 欄位名稱 所表示的意義進行處理

                
    -. 依 轉換檢查表 進行所有檔案一對一的對應，如為 網頁使用的 view or js ，則不轉(進行標記), 如為前端或後端的 程式，轉成 rust
    -. 轉換前，先分析 參考系統 所使用的 framework, 並在 rust 中選出相對應的，並將 選擇結果，記在
        {功能規格目錄}/50.Framework.md
產生 工作用 shell (建立在 backend 的目錄下)

    - 編譯 的 bash shell : util_compile.sh
    - 產生 無限回圈 進行執行的 shell : util_corn-syrup-loop-exec.sh
    - 產生 單一次可執行的 shell: util_corn-syrup.sh
    - all in one (不要參考其他的 shared library )編譯 的 bash shell : util_all-in-one-compile.sh
        編譯後的執行檔儲存在 程式碼目錄 中
    - 重新整理所需 package 並 3產生下載離線版的 shell (download_all_package-cron.sh) 並儲存 於 {共用套件目錄}/modules 中
    - 產生一個 makefile

###
  源碼轉譯檢查表   內容與 參考系統 內的檔案內容可能不一致，檢查，經我確認後再異動
###
檢查   源碼轉譯檢查表   中前端與參考系統內的源碼是否有不一致的地方
 將不一致的地方，更新至    源碼轉譯檢查表   檔中
##
   源碼轉譯檢查表   中所有 check 標示成 [ ] <--未完成 
##
以下 所有應執行的動作 (含 python ) , 所有程式 與命令，都 同意執行，且不要再詢問

##
將 uptime-kuma 的參考 license 加入在 README.md 中
將 README.md 翻成 繁體中文 儲存成 README-tw.md
## 轉譯
以下 所有應執行的動作 (含 python ) , 所有程式 與命令，都 同意執行，且不要再詢問
檢查是否有加入 log library, 沒有，則加入
進行    源碼轉譯檢查表   中 50 個未完成的程式碼，進行所對應 源碼的 轉譯，
    - 轉譯說明
        - 我不要產出空的待補程式碼，請依 所對應的 程式 碼實際轉譯, 產出實際可執行的程式碼
            - 應完整還原 Vue 元件實際行為
            - 通知 provider, 依各 provider 實作欄位、驗證與 payload mapping
            - 分析並加上 實際 route table 與 fallback
            - 加上 繁體中文、英文、日文資源與預設繁體中文 fallback
            - Service Worker 類程式，應加上 install / activate / fetch / cache 行為
        - 相關分析的還輯與演算法，使用 註解 記錄在相對的 method 與程式碼上
        - 有相關程式碼，也可一併產出
        - 相關動作，加上 log debug 顯示相關動作
        - 應再檢查其程式碼的關聯應與 參考系統 內的呼叫關聯一致
        - 更新回    源碼轉譯檢查表   中，與 結果記錄在 Current-status 欄位
    -如果不需轉換，請從參考與系統中直接複製

### 轉譯檢查

以下 所有應執行的動作 (含 python ) , 所有程式 與命令，都 同意執行，且不要再詢問
進行    源碼轉譯檢查表   中 150 個未完成的程式碼，進行結果檢查，
    - 轉譯說明檢查
        - 源碼 與 在 參考系統 中 所對應的 程式 進行 實作的比對，
            找出不一致 架構 或實作方法 (並列出那些 內容 未實作) 不同的程式清單
        - 如 架構一樣再檢查其程式碼的關聯應與 參考系統 內的呼叫關聯一致 -> [x]
        - 更新回    源碼轉譯檢查表   中，與 結果記錄在 Current-status 欄位
    - 進行需補強、待確認後再補碼 的 程式
    - 如果不需轉換，請從參考與系統中直接複製

##
分析 20.doc/15.resumes/TRC-216-SDD-V3-03.00.docx, 

請重新產出一個 文件範本 格式為 markdown , 提供之後 AI 產出的參考文件，使用繁體中文, 儲存在
   {功能規格目錄}/89.sdd-template.md
文件中
### 產出 後端 SDD 文件
分析系統規格 產出 {sdd文件} , 相關規格說明如下:

    - 每個章節，皆應各產生兩段 約 200 字的 說明與描述, 但 附錄 不用
    - 使用 繁體中文

### 補強程式碼
進行需補強、待確認後再補碼 的 程式

## cargo 
從 src/backend 目錄的架構，分析，並產生 合適的 Cargo.toml

### 加 log 
以下 所有應執行的動作 (含 python ) , 所有程式 與命令，都 同意執行，且不要再詢問
檢查是否有加入 log library, 沒有，則加入
未完成 [ ] 的程式碼，在相關運作的 method 中, 就流程，檢查或加上 log debug 顯示相關動作

## 
將 參考系統 非程式碼的 'config', 'docker', 'extra', 'public', 'db', 'test'
的目錄 複製到 工作目錄 下，並依本系統的需求進行調整 

##
前端 執行時，發生 layout 未實做，應是有相關程式未產生，
請定計劃， 讓我確認，並改進

##
依目前系統的架構，更新 AGENTS.md, README.md, README-tw.md

# Code review

以下 所有應執行的動作 (含 python ) , 所有程式 與命令，都 同意執行，且不要再詢問

請你扮演一位資深軟體工程師, 使用以下定義, 
    針對    源碼轉譯檢查表   中未完成 [ ] 的部份，依 各個 phase, 
    進行 程式碼進行全面性的 Code Review

所有產出，將結果 除說明外，相關細節形成表格，加上 確認[] 欄位, 與其他內容
        **🔴 Critical Issues** - Must fix before merge
        **🟡 Suggestions** - Improvements to consider
        **✅ Good Practices** - What's done well

            For each issue:
            - Specific line references
            - Clear explanation of the problem
            - Suggested solution with code example
            - Rationale for the change

            Focus on: ${input:focus:Any specific areas to emphasize in the review?}

            Be constructive and educational in your feedback.
    用
       繁體中文 
    寫入 {工作目錄}/20.doc/72.Code-Review/{yyyyMMdd_hhmmss}-Review-{Phase}.md

相關定義:
    - 相關形態程式應依以下程式性質進行檢查:
        - Generate documentation :
            Include:
            1. Overview: What this module does and why it exists
            2. Quick Start: How to use it in 3 steps or less
            3. API Reference: Every public function with params, returns, and examples
            4. Common Patterns: The 3 most common use cases with code
            5. Gotchas: Edge cases, limitations, and things that will bite you
            6. Related: What other modules this works with

            Write for a developer who's new to this codebase but not new to coding.
        - **Architecture & Design**
            - Design pattern usage
            - Separation of concerns
            - Dependency management
            - Error handling strategy
        - **Testing & Documentation**
            - Test coverage and quality
            - Documentation completeness
            - Comment clarity and necessity
        - **Security Issues**
            Check for:
            1. Injection: SQL, NoSQL, command, LDAP
            2. Auth/AuthZ: Session handling, privilege escalation, token issues
            3. Data exposure: Logging secrets, error messages, API responses
            4. Input validation: Missing sanitization, type coercion, length limits
            5. Cryptography: Weak algorithms, hardcoded secrets, improper key handling

            For each finding:
            - Severity: Critical / High / Medium / Low
            - Attack scenario: How would someone exploit this?
            - Fix: Specific code change needed
            - Reference: Relevant OWASP/CWE if applicable

            Assume an attacker with knowledge of our stack.
        - **Performance & Efficiency**
            Check for:
                - Algorithm complexity
                - Memory usage patterns
                - Database query optimization
                - Unnecessary computations
                - Bugs: Logic errors, off-by-one, null handling, race conditions
                -  Security: Injection risks, auth issues, data exposure
                -  Performance: N+1 queries, unnecessary loops, memory leaks
                -  Maintainability: Naming, complexity, duplication
                -  Edge cases: What inputs would break this?

            For each issue:
                - Severity: Critical / High / Medium / Low
                - Line number or section
                - What's wrong
                - How to fix it

            Be harsh. I'd rather fix issues now than in production. 
        - evaluate my approach.
            Context:
                - Scale: [Expected users/requests/data volume]
                - Team: [Size and experience level]
                - Timeline: [Deadline or runway]
                - Existing stack: [What we already use]

            Evaluate:
                1. What are the top 3 risks with this approach?
                2. What would break first at 10x scale?
                3. What's the simplest version I could ship first?
                4. What alternatives should I consider?
                5. What would you do differently if you had [more time / less time]?

            Be specific. I want tradeoffs, not best practices.
        - audit this code:
            Check for:
                1. Injection: SQL, NoSQL, command, LDAP
                2. Auth/AuthZ: Session handling, privilege escalation, token issues
                3. Data exposure: Logging secrets, error messages, API responses
                4. Input validation: Missing sanitization, type coercion, length limits
                5. Cryptography: Weak algorithms, hardcoded secrets, improper key handling

            For each finding:
                - Severity: Critical / High / Medium / Low
                - Attack scenario: How would someone exploit this?
                - Fix: Specific code change needed
                - Reference: Relevant OWASP/CWE if applicable

            Assume an attacker with knowledge of our stack.
        - performance issues:

            Context:
            - This runs [how often: per request / batch job / etc.]
            - Data size: [typical input size]
            - Current pain point: [what feels slow]

            Find:
            1. Time complexity issues (O(n²) operations, unnecessary loops)
            2. Database problems (N+1 queries, missing indexes, over-fetching)
            3. Memory issues (large allocations, leaks, caching opportunities)
            4. I/O bottlenecks (blocking calls, sequential when could be parallel)
            5. Quick wins (simple changes with big impact)

            For each issue:
            - Impact: High / Medium / Low
            - Current behavior
            - Suggested fix with code
            - Expected improvement

            Focus on the 20% of changes that give 80% of the gains.
        - migrate issue

            Constraints:
                - Must maintain backwards compatibility for [duration]
                - Cannot have downtime longer than [limit]
                - Must preserve [specific data/behavior]

            Generate:
                1. Migration checklist (ordered steps)
                2. Code transformations for common patterns
                3. Breaking changes to watch for
                4. Rollback plan
                5. Validation tests to confirm migration worked

            Start with the riskiest parts first.
        - **Code Quality**

            - Readability and maintainability
            - Proper naming conventions
            - Function/class size and responsibility
            - Code duplication
            - Architecture: What pattern is this? (MVC, microservices, monolith, etc.)
            - Entry points: Where does execution start?
            - Core modules: What are the 5 most important files/folders?
            - Data flow: How does data move through the system?
            - Dependencies: What external services/APIs does this rely on?
            - Red flags: What looks concerning from a maintenance perspective?
            - Where to start: If I need to [specific task], which files should I look at first?

            Explain like I'm a senior dev who's never seen this codebase.
    - 五大評估面向:
        1. ✅ 程式邏輯是否正確、是否有潛在錯誤或邊界條件未處理
        2. 🧹 是否符合的慣用寫法,例如錯誤處理、命名風格、簡潔性
        3. 🔒 是否有安全性問題,例如硬編碼、未處理的錯誤、資源洩漏等
        4. 🚀 是否有效能瓶頸,例如不必要的記憶體分配、重複運算、goroutine 使用不當
        5. 📚 是否具備良好的可讀性與可維護性,包括註解、結構清晰度、模組化程度
    - severity 等級的詳細定義(必須嚴格遵守):
        - high(高風險): 問題可能導致系統崩潰、安全漏洞、資料洩漏、嚴重效能衰退,
            或在生產環境造成重大損失。
            例如:未處理的 nil 指標、SQL 注入風險、無限迴圈。
        - medium(中風險): 問題可能引起偶發錯誤、非最佳實踐,但不立即危險。
            例如:未優化的迴圈、缺少錯誤檢查、輕微資源浪費。
        - low(低風險): 小問題,如命名不一致、缺少註解、多餘空白,但不影響功能或效能。
        - no problem(無問題): 不用輸出到 JSON,所有面向皆符合最佳實踐,無任何可改進點
            或者單元測試的 error 寫底線也是可以,放到 no problem
            例如: req, _ := http.NewRequest("GET", "/ping", nil)
    - JSON 格式規範:
        - 每個檔案最多一個討論
        - 討論行對應該檔案的第一個變更行
        - comment 必須包含該檔案所有重要問題的綜合評論,使用繁體中文
        - issues_found 為陣列,每項為短字串描述主要問題,不可用段落文字
        - severity 分為四個等級:high、medium、low、no problem
        - 只在 JSON 回應中輸出 severity 為 high、medium、low 的檔案
        - 如果檔案評估為 no problem,則不要在 gitlab_discussions 陣列中包含該檔案
    - 在評估每個檔案時,請遵循以下結構化步驟:
        1. 先逐個面向分析問題,並列出證據
        2. 綜合所有問題,決定 severity 等級
        3. 只在有明確證據時提升 severity,避免過度評估
##
進行規劃，確認 以下規格後進行修正
  - 列出 參考系統 後端的所有 API Rest, 將結果用 可 驗證 [ ], 的表格加入 API實做檢查表, 內容最少包含
      - API Endppoint, API Route, 來源目錄，來源檔案，
      - 目的目錄，目的檔案, 說明，目前記錄(產出時應異動)
  - 後端是否有使用 rest 的 framework ，如沒有，加入一個，並依 參考系統 異動至 API實做檢查表
  - 系統啟動，應讀取 .env, 並 由 自訂的 host 進行 listen port(寫入 .env 參數) 的執行
  - 確認前 10 API 是有實做的，並計劃進行確認
##
加個功能
進入系統，如讀取不到 .env ，則產生一個
##
如沒有 log frameowork, 加一個
log debug 顯示 .env 的檔案
預設為 log debug , 也將 log 的型態，記在 .env 中
##
後端的程式上，
選用一個 ORM framework, 並將目前的架構進行套用
##
在所有 API 呼叫時，加上 log debug , 從開始呼叫到回傳，顯示所有步驟與相關內容
##
api_settings api 應依 前端 所有資料進行處理，
應先規格所有要呼叫與傳入的內容 (可能有 12 種), 
依各種截入，與儲存資料，進行 model , ORM, 儲存，載入呼叫， 
與 回傳 json 的內容規劃

##
route /api/settings 對應到的 method api_settings(), 看來沒有實作到，檢查是否有實作，列出實證讓我看一下，如沒有，規劃，並進行更新實作

