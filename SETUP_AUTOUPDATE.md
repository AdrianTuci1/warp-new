# Auto-update setup for Octomus fork -> R2

Activat pe branch-ul `octomus/stage2b`. Commiturile:

```
e611214c feat(build): enable autoupdate feature for OSS builds
8d37c2ae feat(oss): enable autoupdate with R2 releases base URL
30f93bb2 feat(ci): release-to-r2 workflow for fork builds
```

## Ce trebuie configurat manual

### 1. Cloudflare R2 — creare bucket

- Creezi un bucket R2 (ex: `octomus-releases`)
- Creezi un **API Token** R2 cu permisiuni **Read+Write** pe bucket
- Notezi: `Access Key ID`, `Secret Access Key`, `Endpoint URL`

### 2. GitHub Secrets — in repo settings

Adaugi in `Settings → Secrets and variables → Actions`:

| Secret | Valoare |
|---|---|
| `R2_ACCESS_KEY_ID` | Din R2 API Token |
| `R2_SECRET_ACCESS_KEY` | Din R2 API Token |
| `R2_ENDPOINT` | `https://<accountid>.r2.cloudflarestorage.com` |
| `R2_BUCKET` | `octomus-releases` (sau numele bucket-ului) |

### 3. Build environment variable

La compilare, trebuie setat `R2_RELEASES_BASE_URL` ca env var. Acesta devine `releases_base_url` in ChannelConfig embedded in binar.

```
R2_RELEASES_BASE_URL=https://pub-<hash>.r2.dev  (sau URL-ul R2.dev / domeniu custom)
```

Optional: adaugi `R2_RELEASES_BASE_URL` ca GitHub Environment variable, sau il setezi in workflow.

### 4. Cum rulezi un release

1. Mergi la `Actions → Release to R2 → Run workflow`
2. Completezi:
   - **version**: `v1.2.3.456` (format: `v{major}.{date}.{patch}`)
   - **channel**: `oss` (default)
3. Workflow-ul:
   - Construieste macOS DMG pe `macos-latest`
   - Upload la R2: `oss/{version}/OctomusOss.dmg` + `WarpOss.dmg`
   - Upload `channel_versions.json` la radacina bucket-ului

### 5. Structura bucket R2

```
bucket/
  channel_versions.json    <-- spune aplicatiei ce versiune e curenta
  oss/
    v1.2.3.456/
      OctomusOss.dmg
      WarpOss.dmg
```

### 6. Cum functioneaza autoupdate

1. Aplicatia porneste si face polling la fiecare 10min catre serverul Warp
2. Daca serverul Warp nu raspunde, face fallback direct la:
   ```
   {releases_base_url}/channel_versions.json
   ```
   (adica R2 bucket)
3. Citeste `channel_versions.json` si compara `dev.version` cu versiunea locala (`GIT_RELEASE_TAG`)
4. Daca exista versiune noua, descarca de la:
   ```
   {releases_base_url}/oss/{new_version}/OctomusOss.dmg
   ```
5. Instaleaza update-ul (la fel ca Warp oficial)

### 7. Workflow-urile existente (din master)

Pentru CI (PR checks, teste, lint) ai nevoie de workflow-urile din `.github/workflows/` de pe master. Le aduci cu:

```bash
git fetch origin master
git checkout octomus/stage2b
git merge origin/master
```

Workflow-ul `release-to-r2.yml` e deja in branch-ul tau si nu necesita merge.

### 8. In viitor (extra)

Workflow-ul `release-to-r2.yml` poate fi extins cu:
- Build Linux (x86_64 + ARM) via namespace-profile runners
- Build Windows
- Build CLI binaries
- Semnare PGP (Linux packages)
- Notificare Slack
- Generare changelog automata

### Fisiere modificate

| Fisier | Ce s-a schimbat |
|---|---|
| `app/src/bin/oss.rs` | Adaugat `autoupdate_config` cu `releases_base_url` din env var |
| `app/src/autoupdate/mod.rs` | Adaugat `Channel::Oss` in `release_assets_directory_url()` si `fetch_version()` |
| `app/Cargo.toml` | Adaugat `autoupdate` in feature flags default |
| `script/macos/bundle` | Adaugat `autoupdate` la features pentru channel oss |
| `.github/workflows/release-to-r2.yml` | **Creat** — workflow nou de release direct la R2 |
