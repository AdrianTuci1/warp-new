# Octomus Multi-Platform Release Setup

## Variabile necesare (GitHub Secrets + Variables)

### Secrets (Settings → Secrets and variables → Actions → Secrets)

| Secret | Obligatoriu | Descriere | Cum se obtine |
|--------|-------------|-----------|---------------|
| `R2_ENDPOINT` | DA | URL endpoint Cloudflare R2 | `https://<account_id>.r2.cloudflarestorage.com` |
| `R2_ACCESS_KEY_ID` | DA | Access Key ID pentru R2 API | Din R2 API Tokens dashboard |
| `R2_SECRET_ACCESS_KEY` | DA | Secret Access Key pentru R2 API | Din R2 API Tokens dashboard |
| `R2_BUCKET` | DA | Numele bucket-ului R2 | Ex: `octomus-releases` |

### Variables (Settings → Secrets and variables → Actions → Variables)

| Variable | Obligatoriu | Descriere | Exemplu |
|----------|-------------|-----------|---------|
| `R2_RELEASES_BASE_URL` | DA | URL public catre bucket (R2.dev sau custom domain) | `https://pub-1234567890abcdef.r2.dev` sau `https://releases.octomus.dev` |

---

## Cum se obtin credentialele R2

### 1. Creezi un bucket R2

1. Mergi la [Cloudflare Dashboard](https://dash.cloudflare.com) → R2
2. Click "Create bucket"
3. Nume: `octomus-releases` (sau ce preferi)
4. Location: alege regiunea cea mai apropiata de utilizatori (ex: `EU-CENTRAL-1` pentru Europa)

### 2. Creezi un API Token R2

1. Mergi la R2 → Manage R2 API Tokens
2. Click "Create API token"
3. Selecteaza bucket-ul creat
4. Permissions: **Object Read & Write**
5. TTL: No expiration (sau seteaza o data daca preferi)
6. Copy **Access Key ID** si **Secret Access Key** (se arata doar o data!)

### 3. Obtii endpoint-ul

Endpoint-ul este format din Account ID:
```
https://<account_id>.r2.cloudflarestorage.com
```

Gasesti Account ID in pagina R2 Overview.

### 4. Obtii URL-ul public (R2.dev)

1. Mergi la bucket → Settings → Public access
2. Enable R2.dev subdomain
3. URL-ul va fi: `https://pub-<hash>.r2.dev`
4. Sau configurezi un custom domain (ex: `releases.octomus.dev`)

---

## Configurare in GitHub Repository

1. Mergi la `Settings → Secrets and variables → Actions`
2. Adauga cele 4 Secrets (tab-ul Secrets)
3. Adauga variabila `R2_RELEASES_BASE_URL` (tab-ul Variables)

---

## Rulare Release

1. Mergi la `Actions → Release Multi-Platform to R2`
2. Click `Run workflow`
3. Completeaza:
   - **version**: `v1.2.3.456` (format recomandat: `v{major}.{minor}.{patch}.{build}`)
   - **channel**: `oss` (default, sau `dev`/`preview`/`stable`)
4. Workflow-ul va:
   - Build macOS DMG (aarch64) pe `macos-latest`
   - Build Linux AppImage x86_64 pe `ubuntu-latest`
   - Build Linux AppImage aarch64 pe `ubuntu-latest` (cross-compile)
   - Build Windows Installer x64 pe `windows-latest`
   - Build Windows Installer arm64 pe `windows-latest`
   - Upload toate artifactele in R2
   - Genera si upload `channel_versions.json`

---

## Structura bucket-ului R2 dupa release

```
octomus-releases/
  channel_versions.json              # <- Fisierul de autoupdate
  oss/
    v1.2.3.456/
      OctomusOss.dmg                   # macOS
      OctomusOss-x86_64.AppImage       # Linux x86_64
      OctomusOss-aarch64.AppImage      # Linux aarch64
      OctomusOssSetup.exe              # Windows x64
      OctomusOssSetup-arm64.exe        # Windows arm64
```

---

## Cum functioneaza autoupdate

1. Aplicatia porneste si verifica versiunea curenta (din `GIT_RELEASE_TAG` compilat)
2. Face request catre: `{R2_RELEASES_BASE_URL}/channel_versions.json`
3. Compara `version` din JSON cu versiunea locala
4. Daca exista versiune noua, descarca de la:
   - macOS: `{R2_RELEASES_BASE_URL}/oss/{version}/OctomusOss.dmg`
   - Linux: `{R2_RELEASES_BASE_URL}/oss/{version}/OctomusOss-{arch}.AppImage`
   - Windows: `{R2_RELEASES_BASE_URL}/oss/{version}/OctomusOssSetup.exe`
5. Instaleaza update-ul si restart

---

## Note

- **macOS**: Build-ul face doar aarch64 (Apple Silicon) din cauza `--nouniversal`. Pentru universal binary (Intel + ARM), scoate `--nouniversal` din workflow, dar build-ul va dura ~2x mai mult.
- **Linux aarch64**: Foloseste cross-compilation cu `aarch64-linux-gnu-gcc`. Daca intampini erori, poti folosi un self-hosted runner ARM sau un container cu QEMU.
- **Windows**: Build-ul foloseste Inno Setup (`ISCC`) pentru crearea installer-ului. Este instalat automat de `prepare_environment` action.
- **Cache**: Workflow-ul foloseste `rust-cache` pentru a reduce timpul de build la release-uri consecutive.
