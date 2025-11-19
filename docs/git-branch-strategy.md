# Git 브랜치 전략

## 브랜치 구조

- **dev**: 개발 브랜치 (기본 작업 브랜치)
- **staging**: 스테이징 환경 배포 브랜치
- **release**: 프로덕션 릴리즈 브랜치

## 워크플로우

1. **개발 시작**

   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/your-feature-name
   ```

2. **개발 완료 후 dev로 머지**

   ```bash
   git checkout dev
   git merge feature/your-feature-name
   git push origin dev
   ```

3. **Staging 환경 배포**

   ```bash
   git checkout staging
   git merge dev
   git push origin staging
   ```

4. **Production 릴리즈**

   ```bash
   git checkout release
   git merge staging
   git push origin release
   ```

## 브랜치 보호 규칙 (GitHub 설정 권장)

### staging 브랜치

- dev 브랜치에서만 머지 가능
- Pull Request 필수
- 최소 1명의 리뷰 승인 필요

### release 브랜치

- staging 브랜치에서만 머지 가능
- Pull Request 필수
- 최소 1-2명의 리뷰 승인 필요
- CI/CD 테스트 통과 필수

## 기본 브랜치 설정

현재 기본 개발 브랜치: **dev**
