# 🛠️ LaxCI — Local CI Runner (like GitHub Actions, but offline)

**LaxCI** is a blazing-fast, minimal CI runner written in Rust that executes workflows defined in a `laxci.yml` file — just like GitHub Actions, but entirely local.

Use it to test, build, lint, and automate development workflows without relying on any remote CI.

---

## 🚀 Features

- 🧱 Define jobs and steps in a YAML file (`laxci.yml`)
- 🌍 Supports `env` at workflow, job, and step level
- 📁 `working-directory` per job or step
- ✅ Pretty CLI output with emoji and colored logs
- ❌ Validates missing directories and fails gracefully

---

## 📦 Installation

```bash
cargo install laxci
```
### Build from source

```bash
git clone https://github.com/07calc/laxci.git
cd laxci
cargo build --release
cargo install --path .
```

## 📝 Example `laxci.yml`

```yaml
name: Build & Deploy

env:
  GLOBAL_VAR: "Hello from Workflow"

jobs:
  build:
    env:
      BUILD_ENV: "debug"
    steps:
      - name: Build app
        run: 'echo "Building in $BUILD_ENV mode"'

  test:
    needs: [build]
    steps:
      - name: Run tests
        run: 'echo "Testing with $GLOBAL_VAR"'

  deploy:
    needs: [test]
    working_directory: ./scripts
    steps:
      - name: Deploy
        run: 'echo "Deploying from $(pwd)"'

```

### ✅ Output

```bash
▶ Running workflow: Build & Deploy

🔨 Job: build
⚙️  Build app
$ echo "Building in $BUILD_ENV mode"
Building in debug mode
✅ Step completed successfully

🔨 Job: test
⚙️  Run tests
$ echo "Testing with $GLOBAL_VAR"
Testing with Hello from Workflow
✅ Step completed successfully

🔨 Job: deploy
⚙️  Deploy
📁 Working directory: ./scripts
$ echo "Deploying from $(pwd)"
Deploying from /your/project/scripts
✅ Step completed successfully

✅ Workflow completed successfully!

```

## 🔧 Roadmap
- [x] `env` support at workflow, job, and step level
- [x] `working-directory` support
- [] built-in commands like `laxci init`
- [x] `needs` support for job dependencies
- [ ] `if:` conditionals for steps
- [ ] timeouts for steps
- [ ] parallel job execution

