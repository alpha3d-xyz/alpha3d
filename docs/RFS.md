# **프로젝트: 차세대 3D 프로토타이핑 플랫폼 (Alpha3D)**

**개발 요구사항 명세서 (Request for Specification)**

> **Note:** For detailed implementation instructions, database schema, and test specifications, please refer to the [Technical Specification](TECHNICAL_SPEC.md).

## **1\. 프로젝트 개요**

* **프로젝트명:** Alpha3D (알파3D)  
* **목표:** 디자인 전공 대학생 및 초기 창업가를 위한 AI 기반 즉시 견적 및 3D 프린팅 주문 플랫폼 구축  
* **핵심 가치:** Rust 기반의 압도적인 연산 속도를 활용한 실시간 모델 분석 및 사용자 친화적 주문 경험 제공  
* **예상 기간:** 4개월 (MVP 런칭 기준)

## **2\. 기술 스택 (Technology Stack)**

**All-Rust Backend** 전략을 통해 성능, 타입 안정성, 유지보수성을 극대화합니다.

### **2.1 Frontend**

* **Framework:** Vue.js 3 (Composition API, TypeScript)  
* **State Management:** Pinia (가볍고 직관적인 상태 관리)  
* **Styling:** Tailwind CSS (빠른 UI 개발)  
* **3D Visualization:** Three.js (또는 @tresjs/core)  
  * 기능: STL/OBJ 뷰어, 모델 자동 회전, 크기 측정 시각화, 슬라이싱 프리뷰(Lite)  
* **Build Tool:** Vite

### **2.2 Backend (API Server)**

* **Language:** Rust (Stable 최신 버전)  
* **Web Framework:** Axum  
  * 선정 이유: Tokio 런타임 기반의 비동기 처리 능력, 타워(Tower) 미들웨어 생태계 활용, 에러 핸들링의 명확성.  
* **Database Interface:** SQLx (PostgreSQL 비동기 드라이버, 컴파일 타임 쿼리 검증)  
* **API Docs:** Utoipa (Rust 코드 주석 기반 OpenAPI/Swagger 자동 생성)  
* **Authentication:** Argon2 (해싱), JWT (인증 토큰)

### **2.3 Core Service (Alpha3D Engine)**

* **Language:** Rust (Backend와 통합 빌드)  
* **Geometry Kernel:**  
  * parry3d / ncollide: 고성능 3D 충돌 감지 및 형상 분석  
  * stl\_io: STL 파일의 바이너리/ASCII 파싱  
* **Core Features:**  
  * **Mesh Analysis:** 부피(Volume), 표면적(Surface Area), 매니폴드(Manifold) 검사.  
  * **Cost Estimation:** 재료 소모량 \+ 예상 출력 시간 기반 알고리즘.  
  * **Auto-Repair (Optional):** 3D 모델의 구멍(Hole)이나 반전된 면(Inverted Normals) 자동 감지.

### **2.4 Infrastructure**

* **Cloud Provider:** Google Cloud Platform (GCP)  
* **Compute:** Cloud Run (Rust 바이너리를 경량 Docker 이미지로 배포, 오토스케일링 용이)  
* **Database:** Cloud SQL for PostgreSQL  
* **Object Storage:** Cloud Storage (사용자 업로드 3D 파일 보관)  
* **CI/CD:** GitHub Actions (Rust Test 및 Build 파이프라인)

## **3\. 주요 기능 요구사항 (Functional Requirements)**

### **3.1 사용자 (User) \- Alpha3D 서비스**

1. **간편 인증:** 이메일 가입 및 소셜 로그인(Google, Kakao), 학생 인증 배지 부여 시스템.  
2. **스마트 업로드 & 뷰어:**  
   * 대용량 STL/OBJ 파일 업로드 (Drag & Drop).  
   * 웹상에서 별도 프로그램 설치 없이 3D 모델 360도 회전/확대/축소.  
   * 모델의 실제 출력 크기(cm/mm) 시각적 확인.  
3. **실시간 견적 산출 (Instant Quote):**  
   * Rust 엔진이 업로드된 파일을 즉시 분석하여 3초 이내 견적 제시.  
   * 재질(PLA, ABS, Resin), 색상, 후가공 옵션 변경 시 가격 실시간 반응.  
4. **주문 및 결제:** 국내 PG사 연동, 배송지 입력, 주문서 생성.  
5. **진행 상태 추적:** 접수완료 \-\> 검토중 \-\> 출력중(Cam 연동 고려) \-\> 배송중 \-\> 완료.

### **3.2 관리자 (Admin) \- Back Office**

1. **통합 대시보드:** 일별/월별 매출, 주문 건수, 주로 사용되는 재질 통계 시각화.  
2. **주문 처리 워크플로우:** 접수된 주문의 3D 파일 다운로드, 오류 파일 반려 사유 발송, 송장 번호 등록.  
3. **견적 엔진 변수 제어:**  
   * 재료비(원/g), 장비 운용비(원/시간), 마진율(%) 등을 Admin에서 수정하면 즉시 사용자 견적에 반영.

## **4\. 시스템 아키텍처 (System Architecture)**

graph TD  
    User\[User (Browser)\] \--\>|Vue.js SPA| CDN\[CloudFront/CDN\]  
    User \--\>|REST API / Websocket| LB\[Load Balancer\]  
      
    subgraph GCP Cloud  
        LB \--\> API\[Alpha3D Backend (Rust Axum)\]  
          
        API \--\>|Auth/User Data| DB\[(PostgreSQL)\]  
        API \--\>|Model Files| Storage\[Object Storage\]  
          
        subgraph Core Engine (Internal Module)  
            API \--\> Analysis\[Geometry Analysis\]  
            API \--\> Price\[Cost Calculator\]  
            Analysis \--\>|CPU Intensive| API  
        end  
    end

* **특이사항:** Rust Axum 서버 내부에 Core Engine 모듈을 포함하여 단일 바이너리로 배포하거나, 부하가 커질 경우 Core Engine만 별도 마이크로서비스로 분리하기 쉬운 구조 채택.

## **5\. 예상 리소스 및 비용 (한국 시장 기준)**

**Alpha3D** 프로젝트는 Rust라는 고성능 언어를 사용하므로, 일반적인 웹 개발 프로젝트보다 인건비 비중이 높지만 인프라 유지비용은 낮아지는 구조입니다.

### **5.1 추천 팀 구성 (총 4\~5명, 소수 정예)**

1. **PM/기획 (1명):** 서비스 기획, 일정 관리, 정책 정의.  
2. **Frontend (Vue.js) (2명):**  
   * **Senior:** Three.js를 활용한 3D 뷰어 구현 및 최적화 담당.  
   * **Junior:** 일반 UI/UX 컴포넌트 및 API 연동.  
3. **Backend/Core (Rust) (2명):**  
   * **Tech Lead:** Rust 아키텍처 총괄, Axum 서버 설계, Core 엔진 알고리즘 구현. (Rust 숙련도 상)  
   * **Developer:** API 비즈니스 로직 구현, DB 설계 및 연동.

### **5.2 예상 일정 (4개월)**

* **1개월:** 기획 고도화, UI 디자인, Rust Core 엔진 프로토타이핑 (파일 파싱 및 부피 계산 정확도 검증).  
* **2개월:** Frontend 뷰어 개발, Backend API 개발, Core 엔진 통합.  
* **3개월:** 결제 모듈 연동, Admin 페이지 개발, 통합 테스트.  
* **4개월:** 베타 오픈, 실사용자 피드백 반영, 버그 수정 및 최적화.

### **5.3 예상 소요 예산 (인건비 중심)**

*(프리랜서/SI 프로젝트 단위 계약 기준 추산)*

| 항목 | 상세 내용 | 예상 비용 (4개월 총액) |
| :---- | :---- | :---- |
| **인건비** | Rust 리드급 (1명) | 4,800 \~ 5,500만원 |
|  | Rust 중급 (1명) | 3,200 \~ 3,600만원 |
|  | Vue.js 고급 (1명) | 3,200 \~ 3,600만원 |
|  | Vue.js 중급 (1명) | 2,000 \~ 2,400만원 |
|  | 기획/디자인 (1명, 3개월) | 1,500 \~ 1,800만원 |
| **기타** | 서버비용(초기), SW라이선스 등 | 500만원 |
| **총계** | **(VAT 별도)** | **약 1.5억 \~ 1.75억원** |

## **6\. 결론**

**Alpha3D**는 Rust와 Vue.js의 조합을 통해 \*\*'압도적인 속도의 견적 경험'\*\*을 제공함으로써 시장에 진입합니다. 초기 개발 난이도와 비용은 다소 높을 수 있으나, Rust의 안정성과 처리 성능은 서비스가 스케일업(Scale-up)할 때 클라우드 비용 절감과 사용자 신뢰도로 보답할 것입니다.